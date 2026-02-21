// ============ IMPORTS ============
use std::{sync::Mutex, path::{Path, PathBuf}, fs, process::Command};
use zbus::{message::Header, interface, Connection, Proxy};
use tokio::sync::mpsc::{Sender, self};
use iced::futures::StreamExt;
use once_cell::sync::Lazy;
use tiny_skia::Pixmap;
use resvg::usvg;





// ============ CRATES ============
use crate::Message;




// ============ CONSTS/STATICS, ETC... ============
static TRAY_RECEIVER: Lazy<Mutex<Option<mpsc::Receiver<TrayEvent>>>> = Lazy::new(|| Mutex::new(None));





// ============ STRUCT ============
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TraySubscription;

#[derive(Debug, Clone)]
pub enum TrayEvent
{
    ItemRegistered(String),
    Icon
    {
        data: Vec<u8>,
        height: u32,
        width: u32
    },
}

pub struct StatusNotifierWatcher
{
    pub sender: Sender<TrayEvent>,
    pub connection: Connection
}

#[derive(Debug, Clone)]
pub struct MenuItem 
{
    pub _visible: bool,
    pub label: String,
    pub id: i32
}





// ============ FUNCTIONS ============
pub fn tray_stream(_: &TraySubscription) -> impl iced::futures::Stream<Item = Message>
{
    let receiver = TRAY_RECEIVER.lock().unwrap().take().expect("tray receiver already taken");
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(Message::TrayEvent)
}



pub fn start_tray()
{
    let (tx, rx) = mpsc::channel(32);
    *TRAY_RECEIVER.lock().unwrap() = Some(rx);
    tokio::spawn(async move { let _ = start_watcher(tx).await; });
}



#[interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher
{
    async fn register_status_notifier_item(&self, service: &str, #[zbus(header)] header: Header<'_>)
    {
        let sender = header.sender().map(|s| s.to_string()).unwrap_or_default();
        let (dest, path) = if service.starts_with('/')
        {
            (sender, service.to_string())
        }
        else
        {
            (service.to_string(), "/StatusNotifierItem".into())
        };

        let combined = format!("{dest}|{path}");
        println!("\n=== Tray item registered ===\nService Registred: {combined}");

        let _ = self.sender.send(TrayEvent::ItemRegistered(combined.clone())).await;

        match fetch_icon(&self.connection, &combined).await
        {
            Ok(icon) => { let _ = self.sender.send(icon).await; }
            Err(e) => println!("Icon fetch failed: {e}")
        }
    }
}



fn extract_nodes(v: &serde_json::Value, entries: &mut Vec<MenuItem>) 
{
    match v 
    {
        serde_json::Value::Array(arr) => 
        {
            if arr.len() == 3 
            {
                let id = arr[0].as_i64().unwrap_or(0) as i32;
                let props = &arr[1];
                if let Some(label_obj) = props.get("label") 
                {
                    let label = label_obj.get("data").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let visible = props.get("visible").and_then(|v| v.get("data")).and_then(|v| v.as_bool()).unwrap_or(true);
                    let enabled = props.get("enabled").and_then(|v| v.get("data")).and_then(|v| v.as_bool()).unwrap_or(true);
                    let entry_type = props.get("type").and_then(|v| v.get("data")).and_then(|v| v.as_str()).unwrap_or("default");
                    if visible && enabled && entry_type != "separator" { entries.push(MenuItem { id, label, _visible: visible }); }
                }
            }
            for elem in arr { extract_nodes(elem, entries); }
        }
        serde_json::Value::Object(map) => 
        {
            for value in map.values() 
            {
                extract_nodes(value, entries);
            }
        }
        _ => {}
    }
}



pub async fn load_menu(service: &str, menu_path: &str) -> zbus::Result<Vec<MenuItem>> 
{
    let output = Command::new("busctl").args(["--user", "--json=short", "call", service, menu_path, "com.canonical.dbusmenu", "GetLayout", "iias", "0", "1", "0"]).output()?;
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mut entries = Vec::new();
    extract_nodes(&json, &mut entries);
    Ok(entries)
}



pub async fn activate_menu_item(service: &str, menu_path: &str, id: i32) -> zbus::Result<()> 
{
    Command::new("busctl").args(["--user", "call", service, menu_path, "com.canonical.dbusmenu", "Event", "isvu", &id.to_string(), "clicked", "i", "0", "0"]).status()?;
    Ok(())
}



pub async fn start_watcher(sender: mpsc::Sender<TrayEvent>) -> zbus::Result<()>
{
    let connection = Connection::session().await?;
    connection.request_name("org.kde.StatusNotifierWatcher").await?;
    connection.object_server().at("/StatusNotifierWatcher", StatusNotifierWatcher { sender, connection: connection.clone() }).await?;
    println!("\n=== Icebar Watcher Started!!! ===");
    std::future::pending::<()>().await;
    Ok(())
}



pub async fn fetch_icon(conn: &zbus::Connection, combined: &str) -> zbus::Result<TrayEvent>
{
    println!("\n=== Fetching icon for tray item ===");
    let (service, path) = combined.split_once('|').unwrap_or((combined, "/StatusNotifierItem"));
    println!("Service: {service}");
    println!("Path: {path}");
    let proxy = Proxy::new(conn, service, path, "org.kde.StatusNotifierItem").await?;
    println!("Proxy created for {service}");

    // 1️⃣ IconPixmap
    if let Ok(pixmaps) = proxy.get_property::<Vec<(i32, i32, Vec<u8>)>>("IconPixmap").await
    {
        println!("Found IconPixmap with {} candidates", pixmaps.len());
        if let Some((w, h, data)) = pixmaps.into_iter().max_by_key(|(w, h, _)| w * h)
        {
            println!("Using IconPixmap {}x{}", w, h);
            return Ok(TrayEvent::Icon { data, width: w as u32, height: h as u32 });
        }
    }
    else
    {
        println!("No IconPixmap property");
    }

    // 2️⃣ IconThemePath
    let theme_path = proxy.get_property::<String>("IconThemePath").await.ok();
    println!("IconThemePath: {:?}", theme_path);

    // 3️⃣ IconName
    if let Ok(icon_name) = proxy.get_property::<String>("IconName").await
    {
        println!("IconName property: {icon_name}");
        if let Some((bytes, w, h)) = load_icon_with_theme_path(&icon_name, theme_path.as_deref())
        {
            println!("Loaded icon via IconName: {}x{}", w, h);
            return Ok(TrayEvent::Icon { data: bytes, width: w, height: h });
        }
    }
    else
    {
        println!("No IconName property");
    }

    // 4️⃣ AttentionIconName fallback
    if let Ok(icon_name) = proxy.get_property::<String>("AttentionIconName").await
    {
        println!("AttentionIconName property: {icon_name}");
        if let Some((bytes, w, h)) = load_icon_with_theme_path(&icon_name, theme_path.as_deref())
        {
            println!("Loaded icon via AttentionIconName: {}x{}", w, h);
            return Ok(TrayEvent::Icon { data: bytes, width: w, height: h });
        }
    }
    else
    {
        println!("No AttentionIconName property");
    }

    // 5️⃣ .desktop Fallback
    println!("Attempting .desktop fallback...");

    // Try Title property first
    let title = proxy.get_property::<String>("Title").await.ok();
    if let Some(title) = title
    {
        println!("Title from proxy: {title}");
        if let Some((bytes, w, h)) = load_icon_from_desktop(&title)
        {
            println!("Loaded icon via .desktop fallback: {}x{}", w, h);
            return Ok(TrayEvent::Icon { data: bytes, width: w, height: h });
        }
    }
    else
    {
        println!("No Title property available for .desktop fallback, trying service-based guess...");
        if let Some((bytes, w, h)) = load_icon_from_desktop(service)
        {
            println!("Loaded icon via .desktop fallback using service name: {}x{}", w, h);
            return Ok(TrayEvent::Icon { data: bytes, width: w, height: h });
        }
    }

    println!("=== Failed to load any icon for {combined} ===");
    Err(zbus::Error::Failure("No icon available".into()))
}



fn load_icon_from_desktop(name: &str) -> Option<(Vec<u8>, u32, u32)>
{
    println!("Searching .desktop files for app: {name}");
    let mut desktop_paths = vec!
    [
        // System and user installations
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        home::home_dir().map(|h| h.join(".local/share/applications")).unwrap_or_default(),

        // Flatpak standard paths
        home::home_dir().map(|h| h.join(".local/share/flatpak/exports/share/applications")).unwrap_or_default(),
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
    ];

    if let Some(home_path) = home::home_dir()
    {
        let home = home_path.display().to_string();
        let flatpak_app_dirs = vec!
        [
            format!("{home}/.local/share/flatpak/app"),
            format!("/var/lib/flatpak/app"),
        ];
        for base in flatpak_app_dirs
        {
            if let Ok(entries) = fs::read_dir(base)
            {
                for entry in entries.flatten()
                {
                    let path = entry.path().join("current/active/export/share/applications");
                    if path.exists() { desktop_paths.push(path); }
                }
            }
        }
    }

    for dir in desktop_paths
    {
        if !dir.exists() { continue; }
        if let Ok(entries) = fs::read_dir(&dir)
        {
            for entry in entries.flatten()
            {
                let path = entry.path();
                if !path.is_file() { continue; }
                if path.extension().and_then(|e| e.to_str()) != Some("desktop") { continue; }

                if let Ok(content) = fs::read_to_string(&path)
                {
                    let lower_content = content.to_lowercase();
                    if lower_content.contains(&name.to_lowercase())
                    {
                        println!("Found matching .desktop file: {:?}", path);

                        // parse Icon field
                        if let Some(icon_line) = content.lines().find(|l| l.starts_with("Icon="))
                        {
                            let icon_name = icon_line.trim_start_matches("Icon=").trim();
                            println!("Icon field in .desktop: {icon_name}");
                            if let Some(icon) = load_icon_with_theme_path(icon_name, None)
                            {
                                return Some(icon);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No matching .desktop icon found for {name}");
    None
}



fn try_load_icon(path: &std::path::Path) -> Option<(Vec<u8>, u32, u32)>
{
    let bytes = std::fs::read(path).ok()?;
    match path.extension().and_then(|e| e.to_str())
    {
        Some("svg") =>
        {
            let opt = usvg::Options::default();
            let tree = usvg::Tree::from_data(&bytes, &opt).ok()?;
            let size = tree.size().to_int_size();
            let mut pixmap = Pixmap::new(size.width(), size.height())?;
            resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());
            Some((pixmap.data().to_vec(), size.width(), size.height()))
        }
        _ =>
        {
            let img = image::load_from_memory(&bytes).ok()?;
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            Some((rgba.into_raw(), w, h))
        }
    }
}



fn load_icon_with_theme_path(name: &str, theme_path: Option<&str>) -> Option<(Vec<u8>, u32, u32)> 
{
    println!("Trying to load icon: {name} with theme_path: {:?}", theme_path);
    if let Some(base) = theme_path
    {
        if !base.is_empty()
        {
            let base = PathBuf::from(base);
            for size in ["16x16","22x22","24x24","32x32","48x48","scalable"]
            {
                for ext in ["svg","png"] // prefer svg first
                {
                    let candidate = base.join(size).join("apps").join(format!("{name}.{ext}"));
                    if let Some(icon) = try_load_icon(&candidate)
                    {
                        println!("Loaded icon from app theme path: {:?}", candidate);
                        return Some(icon);
                    }
                }
            }
        }
    }

    if let Some(icon) = load_icon_from_theme(name)
    {
        println!("Loaded icon from system/user theme: {name}");
        return Some(icon);
    }

    let home = home::home_dir().expect("Failed to get home directory").display().to_string();
    let flatpak_candidates = 
    [
        format!("{home}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("{home}/.local/share/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
        format!("/var/lib/flatpak/exports/share/icons/hicolor/scalable/apps/{name}.svg"),
        format!("/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps/{name}.png"),
    ];

    for path_str in flatpak_candidates.iter()
    {
        let path = PathBuf::from(path_str);
        if let Some(icon) = try_load_icon(&path)
        {
            println!("Loaded icon from flatpak path: {:?}", path);
            return Some(icon);
        }
    }

    // 4️⃣ Symbolic hicolor fallback
    let symbolic_candidate = PathBuf::from("/usr/share/icons/hicolor/scalable/apps").join(format!("{name}.svg"));
    if let Some(icon) = try_load_icon(&symbolic_candidate)
    {
        println!("Loaded symbolic fallback icon: {:?}", symbolic_candidate);
        return Some(icon);
    }

    // None found
    println!("No icon found for {name}");
    None
}



fn load_icon_from_theme(name: &str) -> Option<(Vec<u8>, u32, u32)>
{
    let exts = ["png","svg","xpm"];
    let mut roots = vec!
    [
        PathBuf::from("/usr/share/icons"),
        PathBuf::from("/usr/local/share/icons"),
        PathBuf::from("/usr/share/pixmaps"),
    ];

    if let Some(home) = home::home_dir()
    {
        roots.push(home.join(".local/share/icons"));
    }

    for root in roots
    {
        if root.exists()
        {
            if let Some(path) = search_icon_recursive(&root, name, &exts)
            {
                println!("Loaded icon from {:?}", path);
                return try_load_icon(&path);
            }
        }
    }

    None
}



fn search_icon_recursive(dir: &Path, name: &str, exts: &[&str]) -> Option<PathBuf>
{
    if let Ok(entries) = fs::read_dir(dir)
    {
        for entry in entries.flatten()
        {
            let path = entry.path();
            if path.is_dir()
            {
                if let Some(found) = search_icon_recursive(&path, name, exts)
                {
                    return Some(found);
                }
            }
            else if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            {
                if stem == name
                {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str())
                    {
                        if exts.contains(&ext)
                        {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}
