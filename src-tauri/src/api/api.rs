use std::borrow::ToOwned;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use futures::executor::block_on;
use lettre::{Message, SmtpTransport, Transport};
use lettre::address::AddressError;
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use sea_orm::*;
use tauri::api::path;

use migration::{Migrator, MigratorTrait};

use crate::{db, model};
use crate::entity::prelude::*;
use crate::entity::template;

#[cfg(target_os = "windows")]
const CONF_PATH_DB: &str = "\\.config\\g_email\\g_email.db";
#[cfg(target_os = "windows")]
const CONF_PATH_FILE: &str = "\\.config\\g_email\\g_email.yaml";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CONF_PATH_DB: &str = "/.config/g_email/g_email.db";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const CONF_PATH_FILE: &str = "/.config/g_email/g_email.yaml";

static mut CONF_PATH: &str = "";

#[tauri::command]
pub fn get_template() -> Result<Vec<template::Model>, ()> {
    init_info().expect("初始化配置信息错误");
    let db = block_on(db::get_db()).expect("获取数据库错误");
    // 查询是否已有数据
    let res: Vec<template::Model> = block_on(Template::find().all(&db)).expect("获取数据失败");
    Ok(res)
}

#[tauri::command]
pub fn save_template(title: String, con: String) -> Result<(), String> {
    let db = block_on(db::get_db()).map_err(|err| err.to_string())?;
    let new_data = template::ActiveModel {
        title: Set(title),
        content: Set(con),
        ..Default::default()
    };
    block_on(Template::insert(new_data).exec(&db)).map_err(|_| "保存数据失败".to_string())?;
    Ok(())
}

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
    let mut db_migration_status = false;
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
        db_migration_status = true;
    }
    // 初始化数据库
    block_on(init_db(db_migration_status)).map_err(|_| "初始化数据库错误".to_string()).unwrap();
    Ok(())
}

// 初始化数据库基础模板
async fn init_db(db_migration_status: bool) -> Result<(), DbErr> {
    let db = db::get_db().await.expect("init db error:");
    if db_migration_status {
        Migrator::up(&db, None).await?;
    }
    // 查询是否已有数据
    let res = Template::find().all(&db).await.expect("init db error:");
    if res.len() == 0 {
        let tm_data = template::ActiveModel {
            title: Set("Default".to_string()),
            content: Set(model::init_temp().default),
            ..Default::default()
        };
        Template::insert_many([tm_data]).exec(&db).await?;
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
    init_info().expect("初始化配置信息错误");
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