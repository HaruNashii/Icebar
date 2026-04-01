// ============ IMPORTS ============
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::{path::PathBuf, pin::Pin};





// ============ CRATES ============
use crate::update::Message;





// ============ FUNCTIONS ============
pub fn config_file_watcher(reload_interval: u64, cli_config: Option<String>) -> iced::Subscription<Message> { iced::Subscription::run_with((reload_interval, cli_config), config_watcher_stream) }



fn config_watcher_stream(data: &(u64, Option<String>)) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let reload_interval = data.0;
    let cli_config = data.1.clone();
    Box::pin(async_stream::stream!
    {
        let config_path = if let Some(user_config_path) = cli_config 
        {
            let path_string = if user_config_path.ends_with(".ron")
            {
                user_config_path
            }
            else
            {
                if user_config_path.ends_with("/")
                {
                    format!("{user_config_path}config.ron")
                }
                else
                {
                    format!("{user_config_path}/config.ron")
                }
            };
            let mut path = PathBuf::new();
            path.push(path_string);
            path
        }
        else
        {
            home::home_dir().unwrap_or_default().join(".config/icebar/config.ron")
        };
        let watch_dir = config_path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| config_path.clone());
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let config_path_for_filter = config_path.clone();

        std::thread::spawn(move ||
        {
            let tx_inner = tx.clone();
            let mut watcher = match recommended_watcher(move |res: notify::Result<notify::Event>|
            {
                if let Ok(event) = res
                {
                    let affects_config = event.paths.iter().any(|p| p == &config_path_for_filter);
                    if !affects_config { return; }

                    match event.kind
                    {
                        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => { let _ = tx_inner.send(()); }
                        _ => {}
                    }
                }
            })
            {
                Ok(w)  => w,
                Err(_) => return,
            };

            if watcher.watch(&watch_dir, RecursiveMode::NonRecursive).is_err()
            {
                return;
            }

            loop { std::thread::park(); }
        });

        while rx.recv().await.is_some()
        {
            tokio::time::sleep(std::time::Duration::from_millis(reload_interval)).await;
            while rx.try_recv().is_ok() {}
            yield Message::ConfigChanged;
        }
    })
}
