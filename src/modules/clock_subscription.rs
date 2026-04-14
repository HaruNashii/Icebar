// ============ IMPORTS ============
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};





// ============ CRATES ============
use crate::update::Message;





// ============ FUNCTIONS ============
pub fn clock_subscription(granularity_ms: u64) -> iced::Subscription<Message> { iced::Subscription::run_with(granularity_ms, clock_stream) }



pub fn clock_stream(granularity_ms: &u64) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let granularity_ms = *granularity_ms;
    Box::pin(async_stream::stream!
    {
        loop
        {
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .subsec_millis() as u64;

            let elapsed_in_window = now_ms % granularity_ms;
            let sleep_ms = granularity_ms - elapsed_in_window;

            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            yield Message::UpdateClock;
        }
    })
}
