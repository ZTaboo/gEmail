use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    pub smtp_service: String,
    pub username: String,
    pub password: String,
}

pub struct TemplateData {
    pub default: String,
}

pub fn init_temp() -> TemplateData {
    let data = TemplateData {
        default: tp_default(),
    };
    return data;
}

fn tp_default() -> String {
    let x = r##"<!DOCTYPE html>
<html>

<head>
    <title>注册确认邮件</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body>
<div style="background-color: #f2f2f2; padding: 30px;">
    <div style="background-color: #333; padding: 10px; text-align: center; color: #fff;">
        <h1>欢迎加入我们！</h1>
        <p>感谢您注册 LearnHacker。</p>
    </div>
    <div style="background-color: #fff; padding: 20px; border-radius: 5px; margin-top: 20px;">
        <h2 style="color: #333;">您的注册验证码</h2>
        <p>您的注册验证码是：<strong>123456</strong></p>
        <p>请在注册页面输入此验证码以完成注册。</p>
    </div>
    <div style="display: flex;justify-content: end;padding-right: 20px;">
        <div>
            <p>祝您使用愉快！</p>
            <p>LearnHacker 团队</p>
            <p style="margin-top: 20px;">如果您未注册，请忽略此邮件。</p>
        </div>
    </div>
</div>
</body>
</html>"##;
    x.to_string()
}
