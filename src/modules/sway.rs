// ============ IMPORTS ============
use swayipc::Connection;





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone)]
pub enum UserSwayAction
{
    ChangeWithIndex(usize),
    MoveNext,
    MovePrev
}





// ============ FUNCTIONS ============

pub fn current_workspace() -> i32
{
    let result_connection = Connection::new();
    if let Ok(mut connection) = result_connection
    {
        let result_workspaces = connection.get_workspaces();
        if let Ok(workspaces) = result_workspaces
        {
            for workspace in workspaces
            {
                if workspace.focused
                {
                    return workspace.num;
                }
            }
        }
    }
    0
}
pub fn workspace_count() -> Vec<i32>
{ 
    let result_connection = Connection::new();
    if let Ok(mut connection) = result_connection
    {
        let result_workspace_data = connection.get_workspaces();
        if let Ok(workspace_data) = result_workspace_data
        {
            let workspace_num: Vec<i32> = workspace_data.iter().map(|item| item.num).collect();
            return workspace_num;
        };
    }
    Vec::new()
}
pub fn change_workspace_sway(action: UserSwayAction)
{
    let result_conn = Connection::new();
    if let Ok(mut conn) = result_conn
    {
        match action 
        {
            UserSwayAction::ChangeWithIndex(index) =>
            {
                let _ = conn.run_command(format!("workspace number {index}"));
            }
            UserSwayAction::MoveNext =>
            {
                let _ = conn.run_command("workspace next");
            }
            UserSwayAction::MovePrev =>
            {
                let _ = conn.run_command("workspace prev");
            }
        }
    }
}
