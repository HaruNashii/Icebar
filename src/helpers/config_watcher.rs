// ============ IMPORTS ============
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::pin::Pin;





// ============ CRATES ============
use crate::update::Message;





// ============ FUNCTIONS ============
pub fn config_file_watcher(reload_interval: u64) -> iced::Subscription<Message>
{
    iced::Subscription::run_with(reload_interval, config_watcher_stream)
}

fn config_watcher_stream(reload_interval: &u64) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let reload_interval = *reload_interval;
    Box::pin(async_stream::stream!
    {
        let config_path = home::home_dir().unwrap_or_default().join(".config/icebar/config.ron");
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        std::thread::spawn(move ||
        {
            let tx_inner = tx.clone();
            let mut watcher = recommended_watcher(move |res: notify::Result<notify::Event>|
            {
                if let Ok(event) = res
                {
                    match event.kind
                    {
                        EventKind::Modify(_) | EventKind::Create(_) =>
                        {
                            let _ = tx_inner.send(());
                        }
                        _ => {}
                    }
                }
            }).expect("Failed to create file watcher");
            watcher.watch(&config_path, RecursiveMode::NonRecursive).expect("Failed to watch config file");
            loop { std::thread::sleep(std::time::Duration::from_secs(60)); }
        });
        while rx.recv().await.is_some()
        {
            tokio::time::sleep(std::time::Duration::from_millis(reload_interval)).await;
            while rx.try_recv().is_ok() {}
            yield Message::ConfigChanged;
        }
    })
}
