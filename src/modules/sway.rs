// ============ IMPORTS ============
use swayipc::Connection;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct UserSwayData
{
    pub workspace_count: usize,
    pub current_workspace: i32
}

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
    let mut connection = Connection::new().unwrap();
    let workspaces = connection.get_workspaces().unwrap();
    workspaces.iter().find(|ws| ws.focused).expect("No focused workspace").num
}
pub fn workspace_count() -> usize
{ 
    let mut connection = Connection::new().unwrap();
    connection.get_workspaces().unwrap().len()
}
pub fn change_workspace(action: UserSwayAction)
{
    let mut conn = Connection::new().expect("Failed To Create Connection With Sway IPC");
    match action 
    {
        UserSwayAction::ChangeWithIndex(index) =>
        {
            conn.run_command(format!("workspace number {index}")).expect("Failed To Change Workspace Of Sway");
        }
        UserSwayAction::MoveNext =>
        {
            conn.run_command("workspace next".to_string()).expect("Failed To Change Workspace Of Sway");
        }
        UserSwayAction::MovePrev =>
        {
            conn.run_command("workspace prev".to_string()).expect("Failed To Change Workspace Of Sway");
        }
    }
}
