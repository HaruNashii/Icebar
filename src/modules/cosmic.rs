// ============ IMPORTS ============
use std::pin::Pin;

use cosmic_client_toolkit::{
    toplevel_info::{ToplevelInfoHandler, ToplevelInfoState},
    workspace::{WorkspaceHandler, WorkspaceState},
    delegate_toplevel_info, delegate_workspace,
    sctk::{
        output::{OutputHandler, OutputState},
        registry::{ProvidesRegistryState, RegistryState},
        delegate_output, delegate_registry,
    },
};
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1;
use wayland_client::{
    Connection, QueueHandle,
    globals::registry_queue_init,
    protocol::wl_output,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_handle_v1;



// ============ CRATES ============
use crate::modules::workspaces::UserWorkspaceAction;
use crate::update::Message;



// ============ WAYLAND APP STATE ============
struct CosmicAppData
{
    registry_state:  RegistryState,
    output_state:    OutputState,
    workspace_state: WorkspaceState,
    toplevel_info:   ToplevelInfoState,
}

impl ProvidesRegistryState for CosmicAppData
{
    fn registry(&mut self) -> &mut RegistryState { &mut self.registry_state }
    cosmic_client_toolkit::sctk::registry_handlers!(OutputState);
}

impl OutputHandler for CosmicAppData
{
    fn output_state(&mut self) -> &mut OutputState { &mut self.output_state }
    fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
}

impl WorkspaceHandler for CosmicAppData
{
    fn workspace_state(&mut self) -> &mut WorkspaceState { &mut self.workspace_state }
    fn done(&mut self) {}
}

impl ToplevelInfoHandler for CosmicAppData
{
    fn toplevel_info_state(&mut self) -> &mut ToplevelInfoState { &mut self.toplevel_info }

    fn new_toplevel(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
    ) {}

    fn update_toplevel(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
    ) {}

    fn toplevel_closed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
    ) {}
}

delegate_output!(CosmicAppData);
delegate_registry!(CosmicAppData);
delegate_workspace!(CosmicAppData);
delegate_toplevel_info!(CosmicAppData);



// ============ FUNCTIONS ============
fn connect() -> Option<(Connection, CosmicAppData, wayland_client::EventQueue<CosmicAppData>)>
{
    let conn               = Connection::connect_to_env().ok()?;
    let (globals, queue)   = registry_queue_init(&conn).ok()?;
    let qh                 = queue.handle();
    let registry           = RegistryState::new(&globals);

    let toplevel_info = match ToplevelInfoState::try_new(&registry, &qh)
    {
        Some(s) => s,
        None    =>
        {
            eprintln!("[icebar] COSMIC: ext-foreign-toplevel-list-v1 not available");
            return None;
        }
    };

    let data = CosmicAppData
    {
        output_state:    OutputState::new(&globals, &qh),
        workspace_state: WorkspaceState::new(&registry, &qh),
        toplevel_info,
        registry_state:  registry,
    };

    Some((conn, data, queue))
}



pub fn current_workspace_and_count() -> (i32, Vec<i32>)
{
    let Some((_, mut data, mut queue)) = connect() else { return (0, vec![]); };

    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    loop
    {
        if queue.roundtrip(&mut data).is_err() { return (0, vec![]); }
        if data.workspace_state.workspaces().next().is_some() { break; }
        if std::time::Instant::now() >= deadline { break; }
    }

    use wayland_protocols::ext::workspace::v1::client::ext_workspace_handle_v1;

    // Sort by raw coordinate (whatever COSMIC's base happens to be), then
    // assign a clean 1-based display index by *position*, not by value.
    // This avoids relying on whether COSMIC's coordinates start at 0 or 1.
    let mut workspaces: Vec<_> = data.workspace_state.workspaces().collect();
    workspaces.sort_by_key(|w| w.coordinates.first().copied().unwrap_or(0));

    let mut current = 0i32;
    let mut indices  = Vec::new();

    for (position, ws) in workspaces.iter().enumerate()
    {
        let display_idx = position as i32 + 1;
        indices.push(display_idx);
        if ws.state.contains(ext_workspace_handle_v1::State::Active)
        {
            current = display_idx;
        }
    }

    (current, indices)
}



pub fn change_workspace_cosmic(action: UserWorkspaceAction)
{
    use wayland_protocols::ext::workspace::v1::client::ext_workspace_handle_v1;

    let Some((_, mut data, mut queue)) = connect() else
    {
        eprintln!("[icebar] COSMIC: could not connect to Wayland");
        return;
    };

    if queue.roundtrip(&mut data).is_err() { return; }
    if queue.roundtrip(&mut data).is_err() { return; }

    let mut workspaces: Vec<_> = data.workspace_state.workspaces().collect();
    workspaces.sort_by_key(|w| w.coordinates.first().copied().unwrap_or(0));

    let current_position = workspaces
        .iter()
        .position(|w| w.state.contains(ext_workspace_handle_v1::State::Active));

    let target_handle = match action
    {
        // id is the 1-based display index — convert to a 0-based position
        UserWorkspaceAction::ChangeWithIndex(id) =>
            workspaces.get((id - 1) as usize).map(|w| w.handle.clone()),

        UserWorkspaceAction::MoveNext =>
            current_position
                .and_then(|pos| workspaces.get(pos + 1))
                .map(|w| w.handle.clone()),

        UserWorkspaceAction::MovePrev =>
            current_position
                .filter(|&pos| pos > 0)
                .and_then(|pos| workspaces.get(pos - 1))
                .map(|w| w.handle.clone()),
    };

    if let Some(handle) = target_handle
    {
        handle.activate();
        if let Ok(manager) = data.workspace_state.workspace_manager().get()
        {
            manager.commit();
        }
        let _ = queue.roundtrip(&mut data);
    }
}



pub fn read_focused_window_cosmic() -> Option<String>
{
    let (_, mut data, mut queue) = connect()?;

    // The toplevel info protocol promotes pending state to current state only
    // after a `Done` event from the manager. A fixed two roundtrips can land
    // before that event arrives, so keep dispatching for a short window.
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    loop
    {
        if queue.roundtrip(&mut data).is_err() { return None; }
        if !data.toplevel_info.toplevels().collect::<Vec<_>>().is_empty()
        {
            break;
        }
        if std::time::Instant::now() >= deadline { break; }
    }

    data.toplevel_info
        .toplevels()
        .find(|t| t.state.contains(&zcosmic_toplevel_handle_v1::State::Activated))
        .map(|t| t.title.clone())
}



pub fn cosmic_event_subscription() -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        yield Message::UpdateCosmicWorkspaces;
        yield Message::UpdateFocusedWindowCosmic;

        loop
        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
            let stop         = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let stop_thread  = stop.clone();

            std::thread::spawn(move ||
            {
                let Some((_, mut data, mut queue)) = connect() else
                {
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    return;
                };

                // Give the compositor a short window to send initial toplevel/workspace
                // state (including the `Done` event that promotes pending → current info)
                // before entering the steady-state dispatch loop.
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
                loop
                {
                    if queue.roundtrip(&mut data).is_err() { return; }
                    if std::time::Instant::now() >= deadline { break; }
                }

                loop
                {
                    if stop_thread.load(std::sync::atomic::Ordering::Relaxed) { break; }

                    match queue.blocking_dispatch(&mut data)
                    {
                        Ok(_) => {}
                        Err(e) =>
                        {
                            eprintln!("[icebar] COSMIC Wayland dispatch error: {e}");
                            break;
                        }
                    }

                    let _ = tx.send(Message::UpdateCosmicWorkspaces);
                    let _ = tx.send(Message::UpdateFocusedWindowCosmic);
                }
            });

            while let Some(msg) = rx.recv().await { yield msg; }

            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            eprintln!("[icebar] COSMIC event stream ended — reconnecting in 2s");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    })
}
