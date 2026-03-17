// ============ IMPORTS ============
use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};
use futures_util::StreamExt;
use futures::stream::BoxStream;
use iced::{Subscription, widget::button};
use async_stream::stream;
use anyhow::Result;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style};
use crate::update::Message;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Debug, Clone)]
pub struct NetworkData
{
    pub connection_type: u8,
    pub network_level: u32,
    pub network_speed: u32,
    pub id: String
}




// ============ FUNCTIONS ============
pub fn network_subscription(no_conn_string: String) -> Subscription<Message> { Subscription::run_with(no_conn_string, network_stream) }
pub fn network_stream(no_conn_string: &String) -> BoxStream<'static, Message>
{
    let no_conn_string = no_conn_string.to_owned();
    stream! 
    {
        loop 
        {
            let connection = match Connection::system().await 
            {
                Ok(c) => c,
                Err(e) => 
                {
                    eprintln!("DBus error: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
            };
    
            if let Ok(Some(data)) = return_network_state(&connection).await 
            {
                println!("\n=== Start Network Module ===");
                println!("Fetched Network Data.\n");
                yield Message::NetworkUpdated(data);
            }
    
            let proxy = match Proxy::new(&connection, "org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", "org.freedesktop.DBus.Properties").await 
            {
                Ok(p) => p,
                Err(e) => 
                {
                    eprintln!("Proxy error: {e}");
                    continue;
                }
            };
    
            let mut signals = match proxy.receive_signal("PropertiesChanged").await 
            {
                    Ok(s) => s,
                    Err(e) => 
                    {
                        eprintln!("Signal error: {e}");
                        continue;
                    }
                };
    
            while signals.next().await.is_some() 
            {
                match  return_network_state(&connection).await 
                {
                    Ok(result_data) =>
                    {
                        match result_data 
                        {
                            Some(data) => yield Message::NetworkUpdated(data),
                            None => yield Message::NetworkUpdated(NetworkData { connection_type: 3, network_level: 0, id: no_conn_string.clone(), network_speed: 0 }) 
                        }
                    },
                    Err(_) => yield Message::NetworkUpdated(NetworkData { connection_type: 3, network_level: 0, id: no_conn_string.clone(), network_speed: 0 }) 
                }
            }
    
            println!("Signal stream ended, reconnecting...");
        }
    }.boxed()
}



async fn get_network_speed<'a>(nm: &Proxy<'a>,  connection: &Connection) -> zbus::Result<u32> 
{
    let primary: OwnedObjectPath = nm.get_property("PrimaryConnection").await?;
    if primary.as_str() == "/" { return Ok(0); }
    let active = Proxy::new(connection, "org.freedesktop.NetworkManager", primary.as_str(), "org.freedesktop.NetworkManager.Connection.Active").await?;
    let devices: Vec<OwnedObjectPath> = active.get_property("Devices").await?;
    let device_path = match devices.first() 
    {
        Some(path) => path,
        None => return Ok(0),
    };
    let device = Proxy::new(connection, "org.freedesktop.NetworkManager", device_path.as_str(), "org.freedesktop.NetworkManager.Device").await?;
    let device_type: u32 = device.get_property("DeviceType").await?;

    match device_type 
    {
        1 => 
        {
            // Ethernet
            let wired = Proxy::new(connection, "org.freedesktop.NetworkManager", device_path.as_str(), "org.freedesktop.NetworkManager.Device.Wired").await?;
            let speed: u32 = wired.get_property("Speed").await?;
            Ok(speed) // Mb/s
        }
        2 => 
        { 
            // Wi-Fi
            let wifi = Proxy::new(connection, "org.freedesktop.NetworkManager", device_path.as_str(), "org.freedesktop.NetworkManager.Device.Wireless").await?;
            let bitrate: u32 = wifi.get_property("Bitrate").await?;
            Ok(bitrate / 1000) // convert Kb/s → Mb/s
        }
        _ => Ok(0)
    }
}



async fn return_network_state(connection: &Connection) -> Result<Option<NetworkData>> 
{
    let nm = Proxy::new(connection, "org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", "org.freedesktop.NetworkManager").await?;
    let network_speed = get_network_speed(&nm, connection).await?;
    let connectivity: u32 = nm.get_property("Connectivity").await?;
    let primary: OwnedObjectPath = nm.get_property("PrimaryConnection").await?;
    if primary.as_str() == "/" { return Ok(None); }
    let active = Proxy::new(connection, "org.freedesktop.NetworkManager", primary.as_str(), "org.freedesktop.NetworkManager.Connection.Active").await?;

    

    let id: String = active.get_property("Id").await?;
    let conn_type: String = active.get_property("Type").await?;

    let connection_type = match conn_type.as_str() 
    {
        "802-3-ethernet" => 1,
        "802-11-wireless" => 2,
        _ => 3,
    };

    Ok(Some(NetworkData 
    {
        connection_type,
        network_level: connectivity,
        network_speed,
        id
    }))
}



pub fn define_network_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{   
    if app.is_showing_alt_network_module
    {
        let hovered =           app.ron_config.alt_network_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.alt_network_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.alt_network_button_pressed_color_rgb;
        let normal =            app.ron_config.alt_network_button_color_rgb;
        let normal_text =       app.ron_config.alt_network_text_color_rgb;
        let border_size =           app.ron_config.alt_network_border_size;
        let border_color_rgb = app.ron_config.alt_network_border_color_rgb;
        let border_radius =    app.ron_config.alt_network_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
    else
    {
        let hovered =           app.ron_config.network_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.network_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.network_button_pressed_color_rgb;
        let normal =            app.ron_config.network_button_color_rgb;
        let normal_text =       app.ron_config.network_text_color_rgb;
        let border_size =           app.ron_config.network_border_size;
        let border_color_rgb = app.ron_config.network_border_color_rgb;
        let border_radius =    app.ron_config.network_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
}



pub fn define_network_text(app: &AppData) -> String
{
    let network_level = match &app.modules_data.network_data.network_level
    {
        4 => &app.network_icons[0],
        3 => &app.network_icons[1],
        2 => &app.network_icons[2],
        _ => &app.network_icons[3],
    };

    let connection_type = match &app.modules_data.network_data.connection_type
    {
        1 => &app.connection_type_icons[0],
        2 => &app.connection_type_icons[1],
        _ => &app.connection_type_icons[2],
    };
    
    let network_speed = match &app.modules_data.network_data.network_speed
    {
        0 => &"?".to_string(),
        _ => &app.modules_data.network_data.network_speed.to_string().replace(" ", "").replace("\n", "")
    };


    if app.is_showing_alt_network_module
    {
        let alt_orientation = &app.ron_config.alt_network_text_orientation;
        let alt_string = app.ron_config.alt_network_module_format.replace("{speed}", network_speed).replace("{level}", network_level).replace("{connection_type}", connection_type).replace("{id}", &app.modules_data.network_data.id);
        orient_text(&alt_string, alt_orientation)
    }
    else
    {
        let orientation = &app.ron_config.network_text_orientation;
        let string = app.ron_config.network_module_format.replace("{speed}", network_speed).replace("{level}", network_level).replace("{connection_type}", connection_type).replace("{id}", &app.modules_data.network_data.id);
        orient_text(&string, orientation)
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::modules::network::NetworkData;
    use iced::{Background, Color};
    use iced::widget::button;
 
    fn make_network_style_app(is_alt: bool) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.is_showing_alt_network_module = is_alt;
        app.ron_config.network_button_color_rgb         = [10, 20, 30];
        app.ron_config.network_button_hovered_color_rgb = [15, 25, 35];
        app.ron_config.network_button_pressed_color_rgb = [5,  10, 15];
        app.ron_config.alt_network_button_color_rgb         = [200, 100, 50];
        app.ron_config.alt_network_button_hovered_color_rgb = [210, 110, 60];
        app.ron_config.alt_network_button_pressed_color_rgb = [190,  90, 40];
        app
    }
 
    #[test]
    fn network_style_active_normal_uses_network_color()
    {
        let style = define_network_style(&make_network_style_app(false), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn network_style_active_alt_uses_alt_color()
    {
        let style = define_network_style(&make_network_style_app(true), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(200, 100, 50))));
    }
 
    #[test]
    fn network_style_normal_and_alt_differ()
    {
        let normal = define_network_style(&make_network_style_app(false), button::Status::Active);
        let alt    = define_network_style(&make_network_style_app(true),  button::Status::Active);
        assert_ne!(normal.background, alt.background);
    }
 
    #[test]
    fn network_style_hovered_uses_hovered_color()
    {
        let style = define_network_style(&make_network_style_app(false), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(15, 25, 35))));
    }
 
    // level=2 icon path was missing from the existing tests
    #[test]
    fn network_text_level_2_uses_third_icon()
    {
        // Re-use the make_app helper already defined in the tests block
        let app = make_app(2, 1, 10, "net");
        assert!(define_network_text(&app).contains("L2"));
    }
 
    #[test]
    fn network_text_level_1_uses_last_icon()
    {
        let app = make_app(1, 1, 10, "net");
        assert!(define_network_text(&app).contains("L0"));
    }
 
    fn make_app(level: u32, conn_type: u8, speed: u32, id: &str) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.modules_data.network_data = NetworkData { network_level: level, connection_type: conn_type, network_speed: speed, id: id.into() };
        app.network_icons = ["L4".into(), "L3".into(), "L2".into(), "L0".into()];
        app.connection_type_icons = ["ETH".into(), "WIFI".into(), "?".into()];
        app.ron_config.network_module_format = "{level}|{connection_type}|{speed}|{id}".into();
        app.ron_config.alt_network_module_format = "ALT:{level}".into();
        app
    }
 
    #[test]
    fn network_text_level_4_uses_first_icon()
    {
        let app = make_app(4, 1, 100, "home");
        let text = define_network_text(&app);
        assert!(text.contains("L4"));
    }
 
    #[test]
    fn network_text_level_3_uses_second_icon()
    {
        let app = make_app(3, 1, 100, "home");
        assert!(define_network_text(&app).contains("L3"));
    }
 
    #[test]
    fn network_text_level_below_2_uses_last_icon()
    {
        let app = make_app(0, 1, 100, "home");
        assert!(define_network_text(&app).contains("L0"));
    }
 
    #[test]
    fn network_text_connection_type_1_uses_ethernet_icon()
    {
        let app = make_app(4, 1, 100, "home");
        assert!(define_network_text(&app).contains("ETH"));
    }
 
    #[test]
    fn network_text_connection_type_2_uses_wifi_icon()
    {
        let app = make_app(4, 2, 100, "home");
        assert!(define_network_text(&app).contains("WIFI"));
    }
 
    #[test]
    fn network_text_connection_type_other_uses_unknown_icon()
    {
        let app = make_app(4, 3, 100, "home");
        assert!(define_network_text(&app).contains("?"));
    }
 
    #[test]
    fn network_text_zero_speed_shows_question_mark()
    {
        let app = make_app(4, 1, 0, "home");
        assert!(define_network_text(&app).contains("?"));
    }
 
    #[test]
    fn network_text_nonzero_speed_shows_numeric()
    {
        let app = make_app(4, 1, 75, "home");
        assert!(define_network_text(&app).contains("75"));
    }
 
    #[test]
    fn network_text_id_substituted_correctly()
    {
        let app = make_app(4, 1, 50, "MyNetwork");
        assert!(define_network_text(&app).contains("MyNetwork"));
    }
 
    #[test]
    fn network_text_alt_module_uses_alt_format()
    {
        let mut app = make_app(4, 1, 50, "home");
        app.is_showing_alt_network_module = true;
        let text = define_network_text(&app);
        assert!(text.starts_with("ALT:"));
    }
 
    #[test]
    fn network_text_normal_module_does_not_use_alt_format()
    {
        let app = make_app(4, 1, 50, "home");
        assert!(!define_network_text(&app).starts_with("ALT:"));
    }
}
