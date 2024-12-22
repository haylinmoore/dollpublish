use std::{fs, path::PathBuf};

const DEFAULT_TEMPLATE: &str = include_str!("./template.html");

#[derive(Clone)]
pub struct Templates {
    default: String,
}

impl Templates {
    pub fn new() -> Self {
        Self {
            default: DEFAULT_TEMPLATE.to_string(),
        }
    }

    pub fn render(&self, data_dir: &PathBuf, username: &str, content: &str) -> String {
        // Check for user template
        let user_template_path = data_dir.join(username).join("template.html");
        let template = if user_template_path.exists() {
            fs::read_to_string(&user_template_path).unwrap_or_else(|_| self.default.clone())
        } else {
            self.default.clone()
        };

        template.replace("!!BODY!!", content)
    }
}
