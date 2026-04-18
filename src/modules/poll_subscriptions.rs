// ============ SMART POLLING SUBSCRIPTIONS ============
use std::pin::Pin;
use std::time::Duration;
use crate::update::Message;
use crate::helpers::style::{orient_text, TextOrientation};
use crate::modules::cpu::{read_cpu_snapshot, compute_cpu_usage, CpuSnapshot};
use crate::modules::ram::read_ram_data;
use crate::modules::disk::read_disk_data;
use crate::modules::cpu_temp::read_cpu_temp;
use crate::modules::network::{read_rx_tx, active_iface_from_proc};



fn format_cpu(usage: f32, fmt: &str, orientation: &TextOrientation) -> String
{
    orient_text(&fmt.replace("{usage}", &format!("{:.0}", usage)), orientation)
}

fn format_ram(used_mb: u64, total_mb: u64, percent: f32, fmt: &str, orientation: &TextOrientation) -> String
{
    orient_text(
        &fmt.replace("{used}",    &used_mb.to_string())
            .replace("{total}",   &total_mb.to_string())
            .replace("{percent}", &format!("{:.0}", percent)),
        orientation
    )
}

fn format_disk(total: u64, free: u64, used: u64, percent: u64, fmt: &str, orientation: &TextOrientation) -> String
{
    orient_text(
        &fmt.replace("{total}",   &(total / 1_073_741_824).to_string())
            .replace("{free}",    &(free  / 1_073_741_824).to_string())
            .replace("{used}",    &(used  / 1_073_741_824).to_string())
            .replace("{percent}", &percent.to_string()),
        orientation
    )
}

fn format_temp(temp: f32, fmt: &str, orientation: &TextOrientation) -> String
{
    orient_text(&fmt.replace("{temp}", &format!("{:.0}", temp)), orientation)
}



#[derive(Clone, Hash, PartialEq, Eq)]
pub struct CpuPollConfig
{
    pub interval_ms: u64,
    pub format:      String,
    pub orientation: TextOrientation
}

pub fn cpu_subscription(cfg: &CpuPollConfig) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let cfg = cfg.clone();
    Box::pin(async_stream::stream!
    {
        let mut prev_snap: Option<CpuSnapshot> = None;
        let mut last_text = String::new();
        let interval = Duration::from_millis(cfg.interval_ms);

        loop
        {
            tokio::time::sleep(interval).await;

            let curr_opt = tokio::task::spawn_blocking(read_cpu_snapshot).await.ok().flatten();
            if let Some(curr) = curr_opt
            {
                let usage = if let Some(prev) = &prev_snap { compute_cpu_usage(prev, &curr) } else { 0.0 };
                prev_snap = Some(curr);

                let text = format_cpu(usage, &cfg.format, &cfg.orientation);
                if text != last_text
                {
                    last_text = text.clone();
                    yield Message::CpuTextChanged(text);
                }
            }
        }
    })
}



#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RamPollConfig
{
    pub interval_ms: u64,
    pub format:      String,
    pub orientation: TextOrientation
}

pub fn ram_subscription(cfg: &RamPollConfig) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let cfg = cfg.clone();
    Box::pin(async_stream::stream!
    {
        let mut last_text = String::new();
        let interval = Duration::from_millis(cfg.interval_ms);

        let data_opt = tokio::task::spawn_blocking(read_ram_data).await.ok().flatten();
        if let Some(d) = data_opt
        {
            let text = format_ram(d.used_mb, d.total_mb, d.percent, &cfg.format, &cfg.orientation);
            if text != last_text
            {
                last_text = text.clone();
                yield Message::RamTextChanged(text);
            }
        }

        loop
        {
            tokio::time::sleep(interval).await;

            let data_opt = tokio::task::spawn_blocking(read_ram_data).await.ok().flatten();
            if let Some(d) = data_opt
            {
                let text = format_ram(d.used_mb, d.total_mb, d.percent, &cfg.format, &cfg.orientation);
                if text != last_text
                {
                    last_text = text.clone();
                    yield Message::RamTextChanged(text);
                }
            }
        }
    })
}



#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DiskPollConfig
{
    pub interval_ms: u64,
    pub format:      String,
    pub orientation: TextOrientation,
    pub mount:       String
}

pub fn disk_subscription(cfg: &DiskPollConfig) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let cfg = cfg.clone();
    Box::pin(async_stream::stream!
    {
        let mut last_text = String::new();
        let interval = Duration::from_millis(cfg.interval_ms);

        let mount = cfg.mount.clone();
        let data_opt = tokio::task::spawn_blocking(move || read_disk_data(&mount)).await.ok().flatten();
        if let Some(d) = data_opt
        {
            let text = format_disk(d.total, d.free, d.used, d.percent, &cfg.format, &cfg.orientation);
            if text != last_text
            {
                last_text = text.clone();
                yield Message::DiskTextChanged(text);
            }
        }

        loop
        {
            tokio::time::sleep(interval).await;

            let mount = cfg.mount.clone();
            let data_opt = tokio::task::spawn_blocking(move || read_disk_data(&mount)).await.ok().flatten();
            if let Some(d) = data_opt
            {
                let text = format_disk(d.total, d.free, d.used, d.percent, &cfg.format, &cfg.orientation);
                if text != last_text
                {
                    last_text = text.clone();
                    yield Message::DiskTextChanged(text);
                }
            }
        }
    })
}



#[derive(Clone, Hash, PartialEq, Eq)]
pub struct CpuTempPollConfig
{
    pub interval_ms: u64,
    pub format:      String,
    pub orientation: TextOrientation
}

pub fn cpu_temp_subscription(cfg: &CpuTempPollConfig) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let cfg = cfg.clone();
    Box::pin(async_stream::stream!
    {
        let mut last_text = String::new();
        let interval = Duration::from_millis(cfg.interval_ms);

        let temp_opt = tokio::task::spawn_blocking(read_cpu_temp).await.ok().flatten();
        if let Some(t) = temp_opt
        {
            let text = format_temp(t, &cfg.format, &cfg.orientation);
            if text != last_text
            {
                last_text = text.clone();
                yield Message::CpuTempTextChanged(text);
            }
        }

        loop
        {
            tokio::time::sleep(interval).await;

            let temp_opt = tokio::task::spawn_blocking(read_cpu_temp).await.ok().flatten();
            if let Some(t) = temp_opt
            {
                let text = format_temp(t, &cfg.format, &cfg.orientation);
                if text != last_text
                {
                    last_text = text.clone();
                    yield Message::CpuTempTextChanged(text);
                }
            }
        }
    })
}



// Bug D fix: iface is no longer stored in the config.  The subscription discovers
// the active interface itself on every tick via active_iface_from_proc(), so it
// works correctly at startup (before NetworkManager has emitted its first update)
// and after config reloads (when the old iface snapshot would be stale).
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct NetworkSpeedPollConfig;

pub fn network_speed_subscription(cfg: &NetworkSpeedPollConfig) -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    let _cfg = cfg.clone();
    Box::pin(async_stream::stream!
    {
        let mut prev: Option<(u64, u64, std::time::Instant)> = None;
        let mut last_rx: u64 = u64::MAX;
        let mut last_tx: u64 = u64::MAX;
        let interval = Duration::from_secs(1);

        loop
        {
            tokio::time::sleep(interval).await;

            // Discover the active iface fresh every iteration so we are never
            // blocked by an empty snapshot captured at startup or reload time.
            let result = tokio::task::spawn_blocking(||
            {
                let iface = active_iface_from_proc()?;
                read_rx_tx(&iface)
            }).await.ok().flatten();

            if let Some((rx, tx)) = result
            {
                let now = std::time::Instant::now();
                if let Some((prev_rx, prev_tx, prev_time)) = prev
                {
                    let elapsed = prev_time.elapsed().as_secs_f64();
                    if elapsed > 0.0
                    {
                        let rx_speed = ((rx.saturating_sub(prev_rx)) as f64 / elapsed) as u64;
                        let tx_speed = ((tx.saturating_sub(prev_tx)) as f64 / elapsed) as u64;

                        if rx_speed != last_rx || tx_speed != last_tx
                        {
                            last_rx = rx_speed;
                            last_tx = tx_speed;
                            yield Message::NetworkSpeedChanged(rx_speed, tx_speed);
                        }
                    }
                }
                prev = Some((rx, tx, now));
            }
        }
    })
}
