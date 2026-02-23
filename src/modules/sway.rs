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
            let result_return_value = workspaces.iter().find(|ws| ws.focused);
            if let Some(return_value) = result_return_value
            {
                return return_value.num;
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
        let mut return_vec = Vec::new();
        if let Ok(workspace_data) = result_workspace_data
        {
            for item in workspace_data 
            {
                return_vec.push(item.id as i32);
            }
            return return_vec;
        };
    }
    Vec::new()
}
pub fn change_workspace(action: UserSwayAction)
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
