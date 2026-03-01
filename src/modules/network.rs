// ============ IMPORTS ============
use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};
use futures_util::{Stream, StreamExt};
use iced::widget::button;
use async_stream::stream;
use anyhow::Result;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::update::Message;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct NetworkSubscription;

#[derive(Default, Debug, Clone)]
pub struct NetworkData
{
    pub connection_type: u8,
    pub network_level: u32,
    pub id: String
}





// ============ FUNCTIONS ============
pub fn network_stream() -> impl Stream<Item = Message> 
{
    stream! 
    {
        let connection = match Connection::system().await 
        {
            Ok(c) => c,
            Err(e) => { eprintln!("DBus error: {e}"); return; }
        };

        // initial update
        if let Ok(Some(data)) = return_network_state(&connection).await 
        {
            yield Message::NetworkUpdated(data);
        }

        let proxy = Proxy::new(&connection, "org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", "org.freedesktop.DBus.Properties").await.unwrap();

        let mut signals = proxy.receive_signal("PropertiesChanged").await.unwrap();

        while signals.next().await.is_some() 
        {
            if let Ok(Some(data)) = return_network_state(&connection).await 
            {
                yield Message::NetworkUpdated(data);
            }
        }
    }
}

async fn return_network_state(connection: &Connection) -> Result<Option<NetworkData>> 
{
    let nm = Proxy::new(connection, "org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", "org.freedesktop.NetworkManager").await?;

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
        id,
    }))
}



pub fn define_network_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{   
    let hovered =           app.ron_config.network_button_hovered_color_rgb;
    let hovered_text =      app.ron_config.network_button_hovered_text_color_rgb;
    let pressed =           app.ron_config.network_button_pressed_color_rgb;
    let normal =            app.ron_config.network_button_color_rgb;
    let normal_text =       app.ron_config.network_button_text_color_rgb;
    let border_size =       app.ron_config.network_border_size;
    let border_color_rgba = app.ron_config.network_border_color_rgba;
    let border_radius =     app.ron_config.network_border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
}



pub fn define_network_text(app: &AppData) -> String
{
    let network_level = match &app.modules_data.network_data.network_level
    {
        4 => &app.ron_config.network_level_format[0],
        3 => &app.ron_config.network_level_format[1],
        2 => &app.ron_config.network_level_format[2],
        _ => &app.ron_config.network_level_format[3],
    };

    let connection_type = match &app.modules_data.network_data.connection_type
    {
        1 => &app.ron_config.network_connection_type_icons[0],
        2 => &app.ron_config.network_connection_type_icons[1],
        _ => &app.ron_config.network_connection_type_icons[2],
    };

    app.ron_config.network_module_format.replace("{level}", network_level).replace("{connection_type}", connection_type).replace("{id}", &app.modules_data.network_data.id)
}
