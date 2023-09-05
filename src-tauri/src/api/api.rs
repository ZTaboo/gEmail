use std::fs;
use std::fs::File;
use std::path::Path;

use tauri::api::path;

#[cfg(target_os = "windows")]
const CONF_PATH_DB: &str = "\\.config\\g_email\\g_email.db";
#[cfg(target_os = "windows")]
const CONF_PATH_FILE: &str = "\\.config\\g_email\\g_email.yaml";

#[cfg(target_os = "linux")]
const CONF_PATH_DB: &str = "/.config/g_email/g_email.db";
#[cfg(target_os = "linux")]
const CONF_PATH_FILE: &str = "/.config/g_email/g_email.yaml";

#[tauri::command]
pub fn init_info() {
    let home_db_path = format!("{}{}", path::home_dir()
        .expect("获取用户目录错误")
        .to_str()
        .expect("PathBuf to str err"), CONF_PATH_DB);
    let home_file_path = format!("{}{}", path::home_dir()
        .expect("获取用户目录错误")
        .to_str()
        .expect("PathBuf to str err"), CONF_PATH_FILE);
    let cfg_db_path = Path::new(home_db_path.as_str());
    let cfg_file_path = Path::new(home_file_path.as_str());
    if let Some(parent) = cfg_file_path.parent() {
        fs::create_dir_all(parent).unwrap()
    };
    if !cfg_file_path.exists() {
        File::create(home_file_path).expect("create config file error");
    };
    // 判断路径是否存在;创建目录
    if let Some(parent) = cfg_db_path.parent() {
        fs::create_dir_all(parent).unwrap()
    };
    if !cfg_db_path.exists() {
        // 创建文件
        File::create(home_db_path).expect("create config db file error");
    };
}