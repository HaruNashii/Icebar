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
        let config_path = home::home_dir()
            .unwrap_or_default()
            .join(".config/icebar/config.ron");

        // Watch the DIRECTORY, not the file.
        // Atomic-save editors (vim, neovim, helix...) write a temp file then
        // rename it over the original — a file watch loses the inode and goes
        // silent. A directory watch always sees the rename land.
        let watch_dir = config_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| config_path.clone());

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let config_path_for_filter = config_path.clone();

        std::thread::spawn(move ||
        {
            let tx_inner = tx.clone();
            let mut watcher = match recommended_watcher(move |res: notify::Result<notify::Event>|
            {
                if let Ok(event) = res
                {
                    // Filter: only react to events that involve our specific file.
                    let affects_config = event.paths.iter().any(|p| p == &config_path_for_filter);
                    if !affects_config { return; }

                    match event.kind
                    {
                        // Modify:    in-place write (nano, echo >)
                        // Create:    some editors truncate+rewrite
                        // Remove:    precedes atomic rename in some editors
                        // Any Rename target landing on our path is caught by
                        // the path filter above — no extra arm needed.
                        EventKind::Modify(_)
                        | EventKind::Create(_)
                        | EventKind::Remove(_) =>
                        {
                            let _ = tx_inner.send(());
                        }
                        _ => {}
                    }
                }
            })
            {
                Ok(w)  => w,
                Err(_) => return, // can't create watcher, exit thread silently
            };

            if watcher.watch(&watch_dir, RecursiveMode::NonRecursive).is_err()
            {
                return; // can't watch dir, exit thread silently
            }

            // Keep the watcher alive. The watcher drops when this thread exits,
            // so we park instead of spinning to avoid wasting CPU.
            loop { std::thread::park(); }
        });

        while rx.recv().await.is_some()
        {
            // Debounce: wait for writes to settle, then drain any
            // extra events that piled up during the wait window.
            tokio::time::sleep(std::time::Duration::from_millis(reload_interval)).await;
            while rx.try_recv().is_ok() {}
            yield Message::ConfigChanged;
        }
    })
}
