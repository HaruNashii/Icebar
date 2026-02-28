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
            let mut workspace_num: Vec<i32> = workspace_data.iter().map(|item| item.num).collect();
            workspace_num.retain(|&x| x != 0);
            return workspace_num;
        };
    }
    Vec::new()
}
pub fn change_workspace_sway(action: UserSwayAction)
{
    let result_conn = Connection::new();
    match result_conn
    {
        Ok(mut conn) =>
        {
            match action 
            {
                UserSwayAction::ChangeWithIndex(index) =>
                {
                    let output = conn.run_command(format!("workspace number {index}"));
                    if let Err(err) = output { println!("Warning!!! Couldn't Switch Workspaces With Index Using SwayIPC\nErr: {err}") }
                }
                UserSwayAction::MoveNext =>
                {
                    let result_workspaces = conn.get_workspaces();
                    if let Ok(workspaces) = result_workspaces
                    {
                        for workspace in workspaces
                        {
                            if workspace.focused
                            {
                                let output = conn.run_command(format!("workspace number {}", workspace.num + 1)).map_err(|e| e.to_string());
                                if let Err(err) = output { println!("Warning!!! Couldn't Switch To The Next Workspaces With SwayIPC\nErr: {err}") };
                            }
                        }
                    }
                }
                UserSwayAction::MovePrev =>
                {
                    let result_workspaces = conn.get_workspaces();
                    if let Ok(workspaces) = result_workspaces
                    {
                        for workspace in workspaces
                        {
                            if workspace.focused
                            {
                                let workspace_to_parse = if workspace.num - 1 <= 0 { 1 } else { workspace.num - 1 };
                                let output = conn.run_command(format!("workspace number {}", workspace_to_parse)).map_err(|e| e.to_string());
                                if let Err(err) = output { println!("Warning!!! Couldn't Switch To The Previous Workspaces WIth SwayIPC\nErr: {err}") };
                            }
                        }
                    }
                }
            }
        }
        Err(err) => println!("\n\n\nFailed To Connect With The SwayIPC!!!\nErr: {err}\n\n\n")
    }
}
