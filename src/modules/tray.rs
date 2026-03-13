// ============ IMPORTS ============
use zbus::{Connection, fdo::DBusProxy, interface, message::Header, object_server::SignalEmitter};
use std::{pin::Pin, collections::{HashMap, HashSet}, process::Command, sync::Mutex};
use iced::{Element, widget::{button, image, text}, futures::{Stream, StreamExt}};
use tokio::sync::mpsc::{self, Sender};
use once_cell::sync::Lazy;





// ============ CRATES ============
use crate::helpers::{icons::fetch_icon, style::{UserStyle, set_style}};
use crate::update::Message;
use crate::AppData;





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
    let maybe_rx = TRAY_RECEIVER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take();

    match maybe_rx
    {
        Some(rx) => Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx).map(Message::TrayEvent)),
        None => Box::pin(futures::stream::pending()),
    }
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
        println!("\n=== Tray item registered ===\nService: '{dest}'\nPath: {path}");
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



pub fn define_tray_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered = app.ron_config.tray_button_hovered_color_rgb;
    let hovered_text = app.ron_config.tray_button_hovered_text_color_rgb;
    let pressed = app.ron_config.tray_button_pressed_color_rgb;
    let normal = app.ron_config.tray_button_color_rgb;
    let normal_text = app.ron_config.tray_button_text_color_rgb;
    let border_size = app.ron_config.tray_border_size;
    let border_color_rgba = app.ron_config.tray_border_color_rgba;
    let border_radius = app.ron_config.tray_border_radius;
    set_style(UserStyle {status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius})
}



pub fn define_tray_icon<'a>(app: &'a AppData, icon: &'a Option<iced::widget::image::Handle>) ->  Element<'a, Message>
{
    let element_to_send: Element<_> = if let Some(icon) = icon 
    {
        image(icon.clone()).width(app.ron_config.tray_icon_size).height(app.ron_config.tray_icon_size).into() 
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
    use iced::{Background, Color};
    use iced::widget::button;
    use serde_json::json;
 
    // ---- extract_nodes ------------------------------------------------------
 
    #[test]
    fn extract_nodes_empty_array_produces_nothing()
    {
        let mut out = Vec::new();
        extract_nodes(&json!([]), &mut out);
        assert!(out.is_empty());
    }
 
    #[test]
    fn extract_nodes_visible_enabled_default_item_extracted()
    {
        let v = json!([
            7,
            { "label": {"data": "Open"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].label, "Open");
        assert_eq!(out[0].id, 7);
    }
 
    #[test]
    fn extract_nodes_invisible_item_skipped()
    {
        let v = json!([
            1,
            { "label": {"data": "Hidden"}, "visible": {"data": false}, "enabled": {"data": true}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert!(out.is_empty());
    }
 
    #[test]
    fn extract_nodes_disabled_item_skipped()
    {
        let v = json!([
            2,
            { "label": {"data": "Grey"}, "visible": {"data": true}, "enabled": {"data": false}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert!(out.is_empty());
    }
 
    #[test]
    fn extract_nodes_separator_type_skipped()
    {
        let v = json!([
            3,
            { "label": {"data": "-"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "separator"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert!(out.is_empty());
    }
 
    #[test]
    fn extract_nodes_missing_visible_defaults_to_true()
    {
        let v = json!([
            4,
            { "label": {"data": "NoVis"}, "enabled": {"data": true}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out.len(), 1);
    }
 
    #[test]
    fn extract_nodes_missing_enabled_defaults_to_true()
    {
        let v = json!([
            5,
            { "label": {"data": "NoEna"}, "visible": {"data": true}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out.len(), 1);
    }
 
    #[test]
    fn extract_nodes_id_parsed_correctly()
    {
        let v = json!([
            42,
            { "label": {"data": "Item"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "default"} },
            []
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out[0].id, 42);
    }
 
    #[test]
    fn extract_nodes_nested_children_both_extracted()
    {
        let v = json!([
            [
                10,
                { "label": {"data": "A"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "default"} },
                []
            ],
            [
                11,
                { "label": {"data": "B"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "default"} },
                []
            ]
        ]);
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out.len(), 2);
        let labels: Vec<&str> = out.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"A") && labels.contains(&"B"));
    }
 
    #[test]
    fn extract_nodes_object_recurses_into_values()
    {
        let v = json!({
            "menu": [
                20,
                { "label": {"data": "Nested"}, "visible": {"data": true}, "enabled": {"data": true}, "type": {"data": "default"} },
                []
            ]
        });
        let mut out = Vec::new();
        extract_nodes(&v, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].label, "Nested");
    }
 
    #[test]
    fn extract_nodes_scalar_values_ignored()
    {
        let mut out = Vec::new();
        extract_nodes(&json!(42),    &mut out);
        extract_nodes(&json!("str"), &mut out);
        extract_nodes(&json!(true),  &mut out);
        extract_nodes(&json!(null),  &mut out);
        assert!(out.is_empty());
    }
 
    // ---- define_tray_style --------------------------------------------------
 
    fn make_tray_app() -> AppData
    {
        let mut app = AppData::default();
        app.ron_config.tray_button_color_rgb         = [10, 20, 30];
        app.ron_config.tray_button_hovered_color_rgb = [50, 60, 70];
        app.ron_config.tray_button_pressed_color_rgb = [80, 90, 100];
        app.ron_config.tray_button_text_color_rgb    = [200, 210, 220];
        app.ron_config.tray_button_hovered_text_color_rgb = [255, 255, 255];
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
    fn tray_style_active_text_color_correct()
    {
        let style = define_tray_style(&make_tray_app(), button::Status::Active);
        assert_eq!(style.text_color, Color::from_rgb8(200, 210, 220));
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
