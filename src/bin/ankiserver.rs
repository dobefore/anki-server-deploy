use ankiserver::ApplicationError;
use ankisyncd::{server_run_account, user::user_list};
use updateaddr::update_syncaddr;
#[actix_web::main]
async fn main() -> Result<(), ApplicationError> {
    update_syncaddr().await?;
    server_run_account().await?;
    // print account info.
    match user_list("auth.db") {
        Ok(users) => {
            if let Some(list) = users {
                println!("当前注册过的账号有：{}", list.join(" "));
            }
        }
        Err(_) => eprintln!("未找到存储账户的数据库auth.db"),
    }
    Ok(())
}
