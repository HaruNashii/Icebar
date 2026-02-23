// ============ IMPORTS ============
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}, process::Command, sync::Mutex};
use zbus::{Connection, Proxy, fdo::DBusProxy, interface, message::Header, object_server::SignalEmitter};
use iced::futures::{Stream, StreamExt};
use tokio::sync::mpsc::{self, Sender};
use once_cell::sync::Lazy;
use tiny_skia::Pixmap;
use std::pin::Pin;





// ============ CRATES ============
use crate::Message;





// ============ STATICS ============
static TRAY_RECEIVER: Lazy<Mutex<Option<mpsc::Receiver<TrayEvent>>>> = Lazy::new(|| Mutex::new(None));
static OWNER_MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static REGISTERED: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));





// ============ ENUM/STRUCT ============
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TraySubscription;

#[derive(Debug, Clone)]
pub enum TrayEvent 
{
    ItemUnregistered(String),
    ItemRegistered(String),
    Icon 
    {
        data: Vec<u8>,
        height: u32,
        width: u32,
    },
}

pub struct StatusNotifierWatcher 
{
    pub sender: Sender<TrayEvent>,
    pub connection: Connection,
}

#[derive(Debug, Clone)]
pub struct MenuItem 
{
    pub _visible: bool,
    pub label: String,
    pub id: i32,
}





// ============ FUNCTIONS ============
pub fn tray_stream(_: &TraySubscription) -> Pin<Box<dyn Stream<Item = Message> + Send>>
{
    let rx = TRAY_RECEIVER.lock().unwrap().take().expect("tray receiver already taken");
    Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx).map(Message::TrayEvent))
}



pub fn start_tray() 
{
    println!("\n=== TRAY ===");
    println!("Starting Tray...");
    match std::env::var("DBUS_SESSION_BUS_ADDRESS") 
    {
        Ok(v) =>
        {
            println!("DBUS_SESSION_BUS_ADDRESS = {}", v);
            if v.is_empty()
            {
                for _ in 0..4
                {
                    println!("\n\n\nWARNING!!!!! DBUS_SESSION_BUS_ADRESS IS EMPTY!!, The Tray will not work, if you started your wm that don't setup\n the dbus variables the tray will not work, you can try the workaround:\nstart your WM/DE with: ```dbus-run-session YOUR_WM/DM```.\n\n\n");
                }
            };
        }
        Err(e) => 
        {
                for _ in 0..4
                {
                    println!("{e}");
                    println!("\n\n\nWARNING!!!!! DBUS_SESSION_BUS_ADRESS IS EMPTY!!, The Tray will not work, if you started your wm that don't setup\n the dbus variables the tray will not work, you can try the workaround:\nstart your WM/DE with: ```dbus-run-session YOUR_WM/DM```.\n\n\n");
                }
        }
    }
    let (tx, rx) = mpsc::channel(32);
    *TRAY_RECEIVER.lock().unwrap() = Some(rx);
    
    tokio::spawn(async move 
    {
        if let Err(e) = start_watcher(tx).await { eprintln!("Watcher failed: {e}"); }
    });
}



#[interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher 
{
    pub async fn register_status_notifier_item(&self, service: &str, #[zbus(header)] header: Header<'_>) 
    {
        let sender = header.sender().map(|s| s.to_string()).unwrap_or_default();
        REGISTERED.lock().unwrap().insert(sender.clone());
        let (dest, path) = if service.starts_with('/') 
        {
            (sender.clone(), service.to_string())
        } 
        else 
        {
            (service.to_string(), "/StatusNotifierItem".into())
        };

        let combined = format!("{dest}|{path}");
        let ctxt = SignalEmitter::new(&self.connection, "/StatusNotifierWatcher").unwrap();
        StatusNotifierWatcher::status_notifier_item_registered(&ctxt, &combined).await.unwrap();
        println!("\n=== Tray item registered ===\n{combined}");
        let _ = self.sender.send(TrayEvent::ItemRegistered(combined.clone())).await;
        OWNER_MAP.lock().unwrap().insert(sender.clone(), combined.clone());
        if let Ok(icon) = fetch_icon(&self.connection, &combined).await 
        {
            let _ = self.sender.send(icon).await;
        }
    }

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> { OWNER_MAP.lock().unwrap().values().cloned().collect() }
    
    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> bool { true }
    
    #[zbus(property)]
    fn protocol_version(&self) -> i32 { 0 }

    #[zbus(signal)]
    async fn status_notifier_item_registered(ctxt: &SignalEmitter<'_>, service: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(ctxt: &SignalEmitter<'_>, service: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_registered(ctxt: &SignalEmitter<'_>) -> zbus::Result<()>;
}



pub async fn start_watcher(sender: Sender<TrayEvent>) -> zbus::Result<()> 
{
    let connection = Connection::session().await?;
    connection.request_name("org.kde.StatusNotifierWatcher").await?;
    connection.object_server().at("/StatusNotifierWatcher", StatusNotifierWatcher { sender: sender.clone(), connection: connection.clone() }).await?;
    let ctxt = SignalEmitter::new(&connection, "/StatusNotifierWatcher")?;
    StatusNotifierWatcher::status_notifier_host_registered(&ctxt).await?;
    println!("\n=== StatusNotifier ===");
    println!("StatusNotifierHost registered");

    use futures_util::StreamExt;
    let dbus = DBusProxy::new(&connection).await.unwrap();
    let mut name_changes = dbus.receive_name_owner_changed().await.unwrap();
    let tx_clone = sender.clone();

    tokio::spawn(async move 
    {
        while let Some(signal) = name_changes.next().await 
        {
            let args = signal.args().unwrap();
            let name = args.name().to_string();
            let new_owner = args.new_owner();
            if new_owner.is_none() 
            {
                let was_registered = REGISTERED.lock().unwrap().remove(&name);
                if was_registered 
                {
                    let combined_opt = OWNER_MAP.lock().unwrap().remove(&name);
                    if let Some(combined) = combined_opt 
                    {
                        let ctxt = SignalEmitter::new(&connection, "/StatusNotifierWatcher").unwrap();
                        StatusNotifierWatcher::status_notifier_item_unregistered(&ctxt, &combined).await.unwrap();
                        let _ = tx_clone.send(TrayEvent::ItemUnregistered(combined)).await;
                    }
                }
            }
        }
    });

    println!("\n=== Icebar Watcher ===");
    println!("Started Successfully!!");
    std::future::pending::<()>().await;
    Ok(())
}



fn extract_nodes(v: &serde_json::Value, out: &mut Vec<MenuItem>) 
{
    match v 
    {
        serde_json::Value::Array(arr) => 
        {
            if let [id, props, ..] = &arr[..] 
            {
                let id = id.as_i64().unwrap_or(0) as i32;
                if let Some(label) = props.get("label").and_then(|v| v.get("data")).and_then(|v| v.as_str())
                {
                    let visible = props.get("visible").and_then(|v| v.get("data")).and_then(|v| v.as_bool()).unwrap_or(true);
                    let enabled = props.get("enabled").and_then(|v| v.get("data")).and_then(|v| v.as_bool()).unwrap_or(true);
                    let ty = props.get("type").and_then(|v| v.get("data")).and_then(|v| v.as_str()).unwrap_or("default");
                    if visible && enabled && ty != "separator" 
                    {
                        out.push(MenuItem {id, label: label.into(), _visible: visible});
                    }
                }
            }

            for e in arr { extract_nodes(e, out) }
        }
        serde_json::Value::Object(map) => { map.values().for_each(|v| extract_nodes(v, out)) }
        _ => {}
    }
}



pub async fn load_menu(service: &str, menu_path: &str) -> zbus::Result<Vec<MenuItem>> 
{
    let output = Command::new("busctl").args(["--user", "--json=short", "call", service, menu_path, "com.canonical.dbusmenu", "GetLayout", "iias", "0", "1", "0", ]).output()?;
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let mut entries = Vec::new();
    extract_nodes(&json, &mut entries);
    Ok(entries)
}



pub async fn activate_menu_item(service: &str, menu_path: &str, id: i32) -> zbus::Result<()> 
{
    Command::new("busctl").args(["--user", "call", service, menu_path, "com.canonical.dbusmenu", "Event", "isvu", &id.to_string(), "clicked", "i", "0", "0", ]).status()?;
    Ok(())
}



pub async fn fetch_icon(conn: &Connection, combined: &str) -> zbus::Result<TrayEvent> 
{
    let (service, path) = combined.split_once('|').unwrap_or((combined, "/StatusNotifierItem"));
    let proxy = Proxy::new(conn, service, path, "org.kde.StatusNotifierItem").await?;
    if let Ok(pixmaps) = proxy.get_property::<Vec<(i32, i32, Vec<u8>)>>("IconPixmap").await && let Some((w, h, data)) = pixmaps.into_iter().max_by_key(|(w, h, _)| w * h)
    {
        return Ok(TrayEvent::Icon {data, width: w as u32, height: h as u32});
    }
    let theme_path = proxy.get_property::<String>("IconThemePath").await.ok();
    let try_name = |name: String| {load_icon_with_theme_path(&name, theme_path.as_deref())};
    if let Ok(name) = proxy.get_property::<String>("IconName").await && let Some((d, w, h)) = try_name(name) 
    {
        return Ok(TrayEvent::Icon { data: d, width: w, height: h });
    }
    if let Ok(name) = proxy.get_property::<String>("AttentionIconName").await && let Some((d, w, h)) = try_name(name) 
    {
        return Ok(TrayEvent::Icon { data: d, width: w, height: h });
    }
    if let Ok(title) = proxy.get_property::<String>("Title").await && let Some(icon) = load_icon_from_desktop(&title) 
    {
        let (d, w, h) = icon;
        return Ok(TrayEvent::Icon { data: d, width: w, height: h });
    }
    Err(zbus::Error::Failure("No icon available".into()))
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



fn load_icon_with_theme_path(name: &str, theme_path: Option<&str>) -> Option<(Vec<u8>, u32, u32)> 
{
    println!("Trying to load icon: {name} with theme_path: {:?}", theme_path);
    if let Some(base) = theme_path && !base.is_empty()
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
        if root.exists() && let Some(path) = search_icon_recursive(&root, name, &exts)
        {
            println!("Loaded icon from {:?}", path);
            return try_load_icon(&path);
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
            if path.is_dir() && let Some(found) = search_icon_recursive(&path, name, exts)
            {
                return Some(found);
            }
            else if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) && stem == name && let Some(ext) = path.extension().and_then(|e| e.to_str()) && exts.contains(&ext)
            {
                return Some(path);
            }
        }
    }
    None
}
