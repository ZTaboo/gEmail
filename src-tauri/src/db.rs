use sea_orm::{Database, DatabaseConnection, DbErr};
use tauri::api::path;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CONF_PATH_DB: &str = "/.config/g_email/g_email.db";
#[cfg(target_os = "windows")]
const CONF_PATH_DB: &str = "\\.config\\g_email\\g_email.db";

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let home_file_path = format!(
        "{}{}",
        path::home_dir()
            .expect("获取用户目录错误")
            .to_str()
            .expect("PathBuf to str err"),
        CONF_PATH_DB
    );
    let db = Database::connect(format!("{}{}", "sqlite://", home_file_path.as_str())).await?;
    Ok(db)
}