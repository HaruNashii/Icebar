// ============ imports ============
use anyhow::Result;
use zbus::{zvariant::OwnedObjectPath, Connection, Proxy};
use futures_util::{Stream, StreamExt};
use async_stream::stream;





// ============ CRATES ============
use crate::Message;





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
