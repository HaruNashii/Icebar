// ============ IMPORTS ============
use zbus::{Connection, fdo::DBusProxy, interface, message::Header, object_server::SignalEmitter};
use iced::{Element, widget::{button, image, text}, futures::Stream};
use std::{pin::Pin, collections::{HashMap, HashSet}, sync::Mutex};
use tokio::sync::mpsc::{self, Sender};
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use zbus::zvariant::Value;
use std::sync::LazyLock;




// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, icons::fetch_icon, style::{UserStyle, set_style, SideOption}};
use crate::update::Message;
use crate::AppData;





// ============ TYPE'S ============
type DBusMenuLayout = (i32, HashMap<String, zbus::zvariant::OwnedValue>, Vec<zbus::zvariant::OwnedValue>);





// ============ STATICS ============
static TRAY_RECEIVER: LazyLock<Mutex<Option<mpsc::Receiver<TrayEvent>>>> = LazyLock::new(|| Mutex::new(None));
static TRAY_STATE: LazyLock<Mutex<TrayState>> = LazyLock::new(|| Mutex::new(TrayState
{
    registered: HashSet::new(),
    owner_map:  HashMap::new(),
}));





// ============ ENUM/STRUCT ============
struct TrayState
{
    registered: HashSet<String>,
    owner_map:  HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TraySubscription;

#[derive(Debug, Clone)]
pub enum TrayEvent 
{
    ItemUnregistered(String),
    ItemRegistered(String),
    Icon 
    {
        combined: String,
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







// ============ CONFIG ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TrayConfig
{
    pub tray_icon_size:                    u32,
    pub tray_spacing:                      u32,
    pub tray_button_size:                  u16,
    pub tray_button_color:                 ColorType,
    pub tray_button_hovered_color:         ColorType,
    pub tray_button_hovered_text_color:    ColorType,
    pub tray_button_pressed_text_color:    ColorType,
    pub tray_button_pressed_color:         ColorType,
    pub tray_border_color:                 ColorType,
    pub tray_border_size:                  f32,
    pub tray_border_radius:                [f32; 4],
    pub tray_side_separator:               Option<SideOption>,
    pub tray_side_separator_color:         ColorType,
    pub tray_side_separator_width:         f32,
    pub tray_side_separator_height:        f32,
    pub tray_button_gradient_color:        Option<Gradient>,
    pub tray_button_hovered_gradient_color: Option<Gradient>,
    pub tray_button_pressed_gradient_color: Option<Gradient>,
    pub tray_button_shadow_color:          Option<ColorType>,
    pub tray_button_shadow_x:              f32,
    pub tray_button_shadow_y:              f32,
    pub tray_button_shadow_blur:           f32,
}

impl Default for TrayConfig
{
    fn default() -> Self
    {
        Self
        {
            tray_icon_size:                    18,
            tray_spacing:                      5,
            tray_button_size:                  5,
            tray_button_color:                 ColorType::RGB([60, 50, 70]),
            tray_button_hovered_color:         ColorType::RGB([110, 40, 80]),
            tray_button_hovered_text_color:    ColorType::RGB([255, 255, 255]),
            tray_button_pressed_text_color:    ColorType::RGB([255, 255, 255]),
            tray_button_pressed_color:         ColorType::RGB([70, 20, 40]),
            tray_border_color:                 ColorType::RGB([90, 70, 100]),
            tray_border_size:                  1.0,
            tray_border_radius:                [3.0, 3.0, 3.0, 3.0],
            tray_side_separator:               None,
            tray_side_separator_color:         ColorType::RGB([75, 75, 75]),
            tray_side_separator_width:         1.,
            tray_side_separator_height:        16.,
            tray_button_gradient_color:        None,
            tray_button_hovered_gradient_color: None,
            tray_button_pressed_gradient_color: None,
            tray_button_shadow_color:          None,
            tray_button_shadow_x:              0.0,
            tray_button_shadow_y:              0.0,
            tray_button_shadow_blur:           0.0,
        }
    }
}

// ============ FUNCTIONS ============
pub fn tray_stream(_: &TraySubscription) -> Pin<Box<dyn Stream<Item = Message> + Send>>
{
    let maybe_rx = TRAY_RECEIVER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take();

    match maybe_rx
    {
        Some(rx) => Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx).map(Message::TrayEvent)),
        None => Box::pin(futures::stream::pending()),
    }
}



pub fn start_tray() 
{
    if TRAY_RECEIVER.lock().unwrap_or_else(|p| p.into_inner()).is_some() 
    {
        println!("\n=== TRAY ===");
        println!("Tray already initialized. Skipping...");
        return;
    }
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
    *TRAY_RECEIVER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(rx);
    
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
        let (dest, path) = if service.starts_with('/') 
        {
            (sender.clone(), service.to_string())
        } 
        else 
        {
            (service.to_string(), "/StatusNotifierItem".into())
        };

        let combined = format!("{dest}|{path}");
        {
            let mut state = TRAY_STATE.lock().unwrap_or_else(|p| p.into_inner());
            state.registered.insert(sender.clone());
            state.owner_map.insert(sender.clone(), combined.clone());
        }
        let ctxt = match SignalEmitter::new(&self.connection, "/StatusNotifierWatcher")
        {
            Ok(c) => c,
            Err(e) => { eprintln!("Failed to create signal emitter: {e}"); return; }
        };
        if let Err(e) = StatusNotifierWatcher::status_notifier_item_registered(&ctxt, &combined).await
        {
            eprintln!("Failed to emit tray signal: {e}");
        }
        println!("\n=== Tray item registered ===\nService: '{dest}'\nPath: {path}");
        let _ = self.sender.send(TrayEvent::ItemRegistered(combined.clone())).await;
        if let Ok(icon) = fetch_icon(&self.connection, &combined).await 
        {
            let _ = self.sender.send(icon).await;
        }
    }

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> { TRAY_STATE.lock().unwrap_or_else(|p| p.into_inner()).owner_map.values().cloned().collect() }
    
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

    let dbus = DBusProxy::new(&connection).await?;
    let existing_owner = dbus.get_name_owner("org.kde.StatusNotifierWatcher".try_into()?).await;

    if existing_owner.is_ok()
    {
        println!("=== TRAY ===");
        println!("StatusNotifierWatcher already owned (likely Plasma). Registering as host instead.");

        // Register ourselves as a StatusNotifierHost so apps know a host exists
        let host_name = "org.kde.StatusNotifierHost-icebar";
        connection.request_name(host_name).await?;

        // Subscribe to item registration signals from the existing watcher
        let watcher = zbus::Proxy::new(&connection, "org.kde.StatusNotifierWatcher", "/StatusNotifierWatcher", "org.kde.StatusNotifierWatcher").await?;

        // Fetch already-registered items
        let items: Vec<String> = watcher.get_property("RegisteredStatusNotifierItems").await.unwrap_or_default();
        for item in items
        {
            println!("\n=== Tray item registered ===\nItem: '{item}'");
            let _ = sender.send(TrayEvent::ItemRegistered(item.clone())).await;
            if let Ok(icon) = fetch_icon(&connection, &item).await
            {
                let _ = sender.send(icon).await;
            }
        }

        // Listen for new registrations
        let mut stream = watcher.receive_signal("StatusNotifierItemRegistered").await?;
        let mut unregister_stream = watcher.receive_signal("StatusNotifierItemUnregistered").await?;
        let tx2 = sender.clone();

        tokio::spawn(async move 
        {
            loop 
            {
                tokio::select! 
                {
                    Some(msg) = stream.next() => 
                    {
                        if let Ok((combined,)) = msg.body().deserialize::<(String,)>() 
                        {
                            let _ = sender.send(TrayEvent::ItemRegistered(combined.clone())).await;
                            if let Ok(icon) = fetch_icon(&connection, &combined).await 
                            {
                                let _ = sender.send(icon).await;
                            }
                        }
                    }
                    Some(msg) = unregister_stream.next() => 
                    {
                        if let Ok((combined,)) = msg.body().deserialize::<(String,)>() 
                        {
                            let _ = tx2.send(TrayEvent::ItemUnregistered(combined)).await;
                        }
                    }
                }
            }
        });

        std::future::pending::<()>().await;
        return Ok(());
    }

    connection.request_name("org.kde.StatusNotifierWatcher").await?;
    connection.object_server().at("/StatusNotifierWatcher", StatusNotifierWatcher { sender: sender.clone(), connection: connection.clone() }).await?;
    let ctxt = SignalEmitter::new(&connection, "/StatusNotifierWatcher")?;
    StatusNotifierWatcher::status_notifier_host_registered(&ctxt).await?;
    println!("\n=== StatusNotifier ===");
    println!("StatusNotifierHost registered");

    let dbus = match DBusProxy::new(&connection).await
    {
        Ok(d) => d,
        Err(e) => { eprintln!("Failed to create DBusProxy: {e}"); return Ok(()); }
    };
    let mut name_changes = match dbus.receive_name_owner_changed().await
    {
        Ok(n) => n,
        Err(e) => { eprintln!("Failed to subscribe to name changes: {e}"); return Ok(()); }
    };
    let tx_clone = sender.clone();

    tokio::spawn(async move 
    {
        while let Some(signal) = name_changes.next().await 
        {
            let args = match signal.args()
            {
                Ok(a) => a,
                Err(e) => { eprintln!("Failed to parse signal args: {e}"); continue; }
            };
            let name = args.name().to_string();
            let new_owner = args.new_owner();
            if new_owner.is_none() 
            {
                let combined_opt = 
                {
                    let mut state = TRAY_STATE.lock().unwrap_or_else(|p| p.into_inner());
                    let was_registered = state.registered.remove(&name);
                    if was_registered { state.owner_map.remove(&name) } else { None }
                };
                if let Some(combined) = combined_opt 
                {
                    let ctxt = match SignalEmitter::new(&connection, "/StatusNotifierWatcher")
                    {
                        Ok(c) => c,
                        Err(e) => { eprintln!("Failed to create signal emitter: {e}"); continue; }
                    };
                    if let Err(e) = StatusNotifierWatcher::status_notifier_item_unregistered(&ctxt, &combined).await
                    {
                        eprintln!("Failed to emit tray unregistered signal: {e}");
                    }
                    let _ = tx_clone.send(TrayEvent::ItemUnregistered(combined)).await;
                }
            }
        }
    });

    println!("\n=== Icebar Watcher ===");
    println!("Started Successfully!!");
    std::future::pending::<()>().await;
    Ok(())
}



fn extract_layout_node(id: i32, props: &HashMap<String, zbus::zvariant::OwnedValue>, children: &[zbus::zvariant::OwnedValue], out: &mut Vec<MenuItem>) 
{
    let get_str = |key: &str| -> Option<String> 
    {
        match &**props.get(key)? 
        {
            Value::Str(s) => Some(s.to_string()),
            _ => None,
        }
    };
    
    let get_bool = |key: &str| -> Option<bool> 
    {
        match &**props.get(key)? 
        {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    };

    if let Some(label) = get_str("label") 
    {
        let visible = get_bool("visible").unwrap_or(true);
        let enabled = get_bool("enabled").unwrap_or(true);
        let ty      = get_str("type").unwrap_or_else(|| "default".into());
        if id != 0 && visible && enabled && ty != "separator" 
        {
            out.push(MenuItem { id, label, _visible: visible });
        }
    }

    for child in children 
    {
        if let Ok((child_id, child_props, child_children)) = DBusMenuLayout::try_from(child.clone()) 
        {
            extract_layout_node(child_id, &child_props, &child_children, out);
        }
    }
}



pub async fn load_tray_menu(service: String, path: String) -> Result<(String, String, Vec<crate::tray::MenuItem>), zbus::Error>
{
    let conn = zbus::Connection::session().await?;
    let proxy = zbus::Proxy::new(&conn, service.as_str(), path.as_str(), "org.kde.StatusNotifierItem").await?;
    let menu_path: zbus::zvariant::OwnedObjectPath = proxy.get_property("Menu").await?;
    let items = crate::tray::load_menu(&service, menu_path.as_str()).await.unwrap_or_default();
    Ok((service, menu_path.to_string(), items))
}


pub async fn load_menu(service: &str, menu_path: &str) -> zbus::Result<Vec<MenuItem>>
{
    let conn = Connection::session().await?;
    let proxy = zbus::Proxy::new(&conn, service, menu_path, "com.canonical.dbusmenu").await?;
    let (_, (root_id, root_props, root_children)): (u32, DBusMenuLayout) = proxy.call("GetLayout", &(0i32, 1i32, Vec::<String>::new())).await?;
    let mut entries = Vec::new();
    extract_layout_node(root_id, &root_props, &root_children, &mut entries);
    Ok(entries)
}



pub async fn activate_menu_item(service: &str, menu_path: &str, id: i32) -> zbus::Result<()>
{
    let conn = Connection::session().await?;
    let proxy = zbus::Proxy::new(&conn, service, menu_path, "com.canonical.dbusmenu").await?;
    proxy.call_noreply("Event", &(id, "clicked", zbus::zvariant::Value::I32(0), 0u32)).await?;
    Ok(())
}



pub fn define_tray_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered = app.ron_config.tray.tray_button_hovered_color;
    let hovered_text = app.ron_config.tray.tray_button_hovered_text_color;
    let pressed_text = app.ron_config.tray.tray_button_pressed_text_color;
    let pressed = app.ron_config.tray.tray_button_pressed_color;
    let normal = app.ron_config.tray.tray_button_color;
    let normal_text = ColorType::RGB([255, 255, 255]);
    let border_size = app.ron_config.tray.tray_border_size;
    let border_color = app.ron_config.tray.tray_border_color;
    let border_radius = app.ron_config.tray.tray_border_radius;
    set_style(UserStyle {status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, hovered_gradient: app.ron_config.tray.tray_button_hovered_gradient_color.clone(), normal_gradient: app.ron_config.tray.tray_button_gradient_color.clone(), pressed_gradient: app.ron_config.tray.tray_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.tray.tray_button_shadow_color, shadow_x: app.ron_config.tray.tray_button_shadow_x, shadow_y: app.ron_config.tray.tray_button_shadow_y, shadow_blur: app.ron_config.tray.tray_button_shadow_blur })
}



pub fn define_tray_icon<'a>(app: &'a AppData, icon: &'a Option<iced::widget::image::Handle>) ->  Element<'a, Message>
{
    let element_to_send: Element<_> = if let Some(icon) = icon 
    {
        image(icon.clone()).width(app.ron_config.tray.tray_icon_size).height(app.ron_config.tray.tray_icon_size).into() 
    } 
    else
    { 
        text("?").into() 
    };
    element_to_send
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use zbus::zvariant::{OwnedValue, Value};
    use iced::{widget::button, Background, Color};
 
    // ---- extract_nodes ------------------------------------------------------

    fn str_val(s: &str) -> OwnedValue { OwnedValue::try_from(Value::Str(s.into())).unwrap() }
    fn bool_val(b: bool) -> OwnedValue { OwnedValue::try_from(Value::Bool(b)).unwrap() }

    fn make_props(label: &str, visible: bool, enabled: bool, ty: &str) -> HashMap<String, OwnedValue>
    {
        let mut m = HashMap::new();
        m.insert("label".into(),   str_val(label));
        m.insert("visible".into(), bool_val(visible));
        m.insert("enabled".into(), bool_val(enabled));
        m.insert("type".into(),    str_val(ty));
        m
    }

    #[test]
    fn extract_layout_visible_enabled_default_item_extracted()
    {
        let mut out = Vec::new();
        extract_layout_node(7, &make_props("Open", true, true, "default"), &[], &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].label, "Open");
        assert_eq!(out[0].id, 7);
    }

    #[test]
    fn extract_layout_invisible_item_skipped()
    {
        let mut out = Vec::new();
        extract_layout_node(1, &make_props("Hidden", false, true, "default"), &[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn extract_layout_disabled_item_skipped()
    {
        let mut out = Vec::new();
        extract_layout_node(2, &make_props("Grey", true, false, "default"), &[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn extract_layout_separator_skipped()
    {
        let mut out = Vec::new();
        extract_layout_node(3, &make_props("-", true, true, "separator"), &[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn extract_layout_root_id_zero_skipped()
    {
        // id=0 is the invisible root node, must never appear in results
        let mut out = Vec::new();
        extract_layout_node(0, &make_props("Root", true, true, "default"), &[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn extract_layout_missing_visible_defaults_to_true()
    {
        let mut props = HashMap::new();
        props.insert("label".into(),   str_val("NoVis"));
        props.insert("enabled".into(), bool_val(true));
        props.insert("type".into(),    str_val("default"));
        let mut out = Vec::new();
        extract_layout_node(4, &props, &[], &mut out);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn extract_layout_missing_enabled_defaults_to_true()
    {
        let mut props = HashMap::new();
        props.insert("label".into(),   str_val("NoEna"));
        props.insert("visible".into(), bool_val(true));
        props.insert("type".into(),    str_val("default"));
        let mut out = Vec::new();
        extract_layout_node(5, &props, &[], &mut out);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn extract_layout_no_label_produces_nothing()
    {
        let mut props = HashMap::new();
        props.insert("visible".into(), bool_val(true));
        props.insert("enabled".into(), bool_val(true));
        let mut out = Vec::new();
        extract_layout_node(6, &props, &[], &mut out);
        assert!(out.is_empty());
    }

 
    // ---- define_tray_style --------------------------------------------------
 
    fn make_tray_app() -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.tray.tray_button_color = ColorType::RGB([10, 20, 30]);
        app.ron_config.tray.tray_button_hovered_color = ColorType::RGB([50, 60, 70]);
        app.ron_config.tray.tray_button_pressed_color = ColorType::RGB([80, 90, 100]);
        app.ron_config.tray.tray_button_hovered_text_color = ColorType::RGB([255, 255, 255]);
        app
    }
 
    #[test]
    fn tray_style_active_uses_tray_normal_color()
    {
        let style = define_tray_style(&make_tray_app(), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn tray_style_hovered_uses_tray_hovered_color()
    {
        let style = define_tray_style(&make_tray_app(), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }
 
    #[test]
    fn tray_style_pressed_uses_tray_pressed_color()
    {
        let style = define_tray_style(&make_tray_app(), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }
 
    #[test]
    fn tray_style_all_statuses_produce_background()
    {
        let app = make_tray_app();
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = define_tray_style(&app, status);
            assert!(style.background.is_some(), "Expected background for {:?}", status);
        }
    }
}
