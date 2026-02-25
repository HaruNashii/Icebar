// ============ IMPORTS ============
use std::{fs, path::{Path, PathBuf}};
use zbus::{Connection, Proxy};
use tiny_skia::Pixmap;





// ============ CRATES ============
use crate::TrayEvent;





// ============ FUNCTIONS ============
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



pub fn try_load_icon(path: &std::path::Path) -> Option<(Vec<u8>, u32, u32)>
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



pub fn load_icon_from_desktop(name: &str) -> Option<(Vec<u8>, u32, u32)>
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



pub fn load_icon_with_theme_path(name: &str, theme_path: Option<&str>) -> Option<(Vec<u8>, u32, u32)> 
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



pub fn load_icon_from_theme(name: &str) -> Option<(Vec<u8>, u32, u32)>
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



pub fn search_icon_recursive(dir: &Path, name: &str, exts: &[&str]) -> Option<PathBuf>
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
