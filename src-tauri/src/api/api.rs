use std::borrow::ToOwned;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use lettre::address::AddressError;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;

use tauri::api::path;

use crate::model;

#[cfg(target_os = "windows")]
const CONF_PATH_DB: &str = "\\.config\\g_email\\g_email.db";
#[cfg(target_os = "windows")]
const CONF_PATH_FILE: &str = "\\.config\\g_email\\g_email.yaml";

#[cfg(target_os = "linux")]
const CONF_PATH_DB: &str = "/.config/g_email/g_email.db";
#[cfg(target_os = "linux")]
const CONF_PATH_FILE: &str = "/.config/g_email/g_email.yaml";

static mut CONF_PATH: &str = "";

unsafe fn init_path() {
    let home_file_path = format!(
        "{}{}",
        path::home_dir()
            .expect("获取用户目录错误")
            .to_str()
            .expect("PathBuf to str err"),
        CONF_PATH_FILE
    );
    CONF_PATH = Box::leak(home_file_path.to_owned().into_boxed_str())
}

//  初始化配置文件
#[tauri::command]
pub fn init_info() -> Result<(), String> {
    unsafe { init_path() };
    let home_db_path = format!(
        "{}{}",
        path::home_dir()
            .expect("获取用户目录错误")
            .to_str()
            .expect("PathBuf to str err"),
        CONF_PATH_DB
    );
    let home_file_path = format!(
        "{}{}",
        path::home_dir()
            .expect("获取用户目录错误")
            .to_str()
            .expect("PathBuf to str err"),
        CONF_PATH_FILE
    );
    let cfg_db_path = Path::new(home_db_path.as_str());
    let cfg_file_path = Path::new(home_file_path.as_str());
    if let Some(parent) = cfg_file_path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    if !cfg_file_path.exists() {
        File::create(home_file_path).map_err(|err| err.to_string())?;
        init_yaml().map_err(|e| e.to_string())?;
    }
    // 判断路径是否存在;创建目录
    if let Some(parent) = cfg_db_path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    if !cfg_db_path.exists() {
        // 创建文件
        File::create(home_db_path).map_err(|err| err.to_string())?;
    }
    Ok(())
}

// 初始化配置文件基础信息
fn init_yaml() -> Result<(), String> {
    let data = model::EmailConfig {
        smtp_service: "".to_string(),
        username: "".to_string(),
        password: "".to_string(),
    };
    let yaml = serde_yaml::to_string(&data);
    return match yaml {
        Ok(res) => {
            let file_path = format!(
                "{}{}",
                path::home_dir()
                    .expect("获取用户目录错误")
                    .to_str()
                    .expect("PathBuf to str err"),
                CONF_PATH_FILE
            );
            let mut file = File::options()
                .write(true)
                .read(true)
                .create(true)
                .open(file_path)
                .expect("open file error");
            file.write_all(&mut res.as_bytes()).expect("写入信息错误");
            Ok(())
        }
        Err(_) => Err("序列化字符串错误".into()),
    };
}

#[tauri::command]
pub fn save_email_info(smtp_server: String, username: String, password: String)
                       -> Result<(), String> {
    let data = model::EmailConfig {
        smtp_service: smtp_server,
        username,
        password,
    };
    let yaml_str = serde_yaml::to_string(&data).unwrap();
    unsafe {
        let mut file = File::options()
            .write(true)
            .read(true)
            .create(true)
            .open(CONF_PATH
                .to_string())
            .map_err(|err| err.to_string())?;
        file.write_all(yaml_str.as_bytes())
            .map_err(|_| { "写入信息错误".to_string() })?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_yaml_init() -> Result<model::EmailConfig, String> {
    let file: String;
    unsafe {
        init_path();
        file = fs::read_to_string(CONF_PATH).map_err(|e| e.to_string())?;
    }
    let data: model::EmailConfig = serde_yaml::from_str(&file).map_err(|e| e.to_string())?;
    Ok(data)
}

#[tauri::command]
pub fn send_email(to_address: String, subject: String, body: String) -> Result<(), String> {
    let file: String;
    unsafe {
        init_path();
        file = fs::read_to_string(CONF_PATH).map_err(|e| e.to_string())?;
    }
    let data: model::EmailConfig = serde_yaml::from_str(&file).map_err(|e| e.to_string())?;
    let username = data.username.parse().map_err(|err: AddressError| err.to_string())?;
    let to = to_address.parse().map_err(|err: AddressError| err.to_string())?;
    let email = Message::builder()
        .from(username)
        .to(to)
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            "".to_owned(),
            body.to_owned(),
        ))
        .map_err(|err| err.to_string())?;
    let creds = Credentials::new(data.username.to_owned(), data.password.to_owned());
    let mailer = SmtpTransport::relay(data.smtp_service.as_str())
        .unwrap()
        .credentials(creds)
        .build();
    mailer.send(&email).map_err(|err| err.to_string())?;
    Ok(())
}