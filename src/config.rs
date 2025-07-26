use std::process::Command;

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    pub label: String,
    pub action: String,
    pub text: String,
    pub keybind: String,
}

#[derive(Debug)]
pub struct Config {
    pub buttons: Vec<ButtonConfig>,
}

impl Config {
    pub fn load_from_kdl() -> Result<Self, Box<dyn std::error::Error>> {
        let kdl_content = include_str!("../layout.kdl");
        let document = kdl_content.parse::<kdl::KdlDocument>()?;
        
        let mut buttons = Vec::new();
        
        for node in document.nodes() {
            if node.name().value() == "button" {
                if let Some(label) = node.entries().get(0) {
                    let label = label.value().as_string().unwrap_or("").to_string();
                    
                    let mut action = String::new();
                    let mut text = String::new();
                    let mut keybind = String::new();
                    
                    if let Some(children) = node.children() {
                        for child in children.nodes() {
                            match child.name().value() {
                                "action" => {
                                    if let Some(value) = child.entries().get(0) {
                                        action = value.value().as_string().unwrap_or("").to_string();
                                    }
                                }
                                "text" => {
                                    if let Some(value) = child.entries().get(0) {
                                        text = value.value().as_string().unwrap_or("").to_string();
                                    }
                                }
                                "keybind" => {
                                    if let Some(value) = child.entries().get(0) {
                                        keybind = value.value().as_string().unwrap_or("").to_string();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    buttons.push(ButtonConfig {
                        label,
                        action,
                        text,
                        keybind,
                    });
                }
            }
        }
        
        Ok(Config { buttons })
    }
}

pub fn execute_action(action: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 处理环境变量替换
    let action = action.replace("$USER", &std::env::var("USER").unwrap_or_default());
    
    // 分割命令和参数
    let parts: Vec<&str> = action.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".into());
    }
    
    let mut cmd = Command::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    
    let output = cmd.output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", error_msg).into());
    }
    
    Ok(())
}
