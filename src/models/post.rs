use super::metadata::Metadata;
use crate::error::{AppError, Result};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct PostMetadata {
    pub name: String,
    pub path: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub name: String,
    pub path: String,
    pub metadata: Metadata,
    pub content: String,
    pub attachments: Option<HashMap<String, String>>,
}

impl Post {
    pub async fn save(&self, data_dir: &PathBuf, username: &str, id: &str) -> Result<()> {
        let user_dir = data_dir.join(username).join(id);
        fs::create_dir_all(&user_dir).map_err(|e| AppError::Internal(e.to_string()))?;

        // Save metadata
        let metadata = PostMetadata {
            name: self.name.clone(),
            path: self.path.clone(),
            extra: self.metadata.extra.clone(),
        };

        let metadata_path = user_dir.join("metadata.json");
        fs::write(
            metadata_path,
            serde_json::to_string_pretty(&metadata).unwrap(),
        )
        .map_err(|e| AppError::Internal(e.to_string()))?;

        // Save content
        let content_path = user_dir.join("content.md");
        fs::write(content_path, &self.content).map_err(|e| AppError::Internal(e.to_string()))?;

        // Save attachments
        if let Some(attachments) = &self.attachments {
            let attachments_dir = user_dir.join("attachments");
            fs::create_dir_all(&attachments_dir).map_err(|e| AppError::Internal(e.to_string()))?;

            for (filename, content) in attachments {
                let file_path = attachments_dir.join(filename);
                fs::write(file_path, base64::decode(content).unwrap())
                    .map_err(|e| AppError::Internal(e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn load(data_dir: &PathBuf, username: &str, id: &str) -> Result<Self> {
        let user_dir = data_dir.join(username).join(id);
        let metadata_path = user_dir.join("metadata.json");
        let content_path = user_dir.join("content.md");

        if !metadata_path.exists() || !content_path.exists() {
            return Err(AppError::NotFound);
        }

        // Load metadata
        let metadata_str =
            fs::read_to_string(metadata_path).map_err(|e| AppError::Internal(e.to_string()))?;
        let post_metadata: PostMetadata =
            serde_json::from_str(&metadata_str).map_err(|e| AppError::Internal(e.to_string()))?;

        // Load content
        let content =
            fs::read_to_string(content_path).map_err(|e| AppError::Internal(e.to_string()))?;

        // Load attachments if they exist
        let attachments_dir = user_dir.join("attachments");
        let attachments = if attachments_dir.exists() {
            let mut attachments_map = HashMap::new();
            for entry in
                fs::read_dir(attachments_dir).map_err(|e| AppError::Internal(e.to_string()))?
            {
                let entry = entry.map_err(|e| AppError::Internal(e.to_string()))?;
                let filename = entry.file_name();
                let filename_str = filename.to_string_lossy().into_owned();
                let content =
                    fs::read(entry.path()).map_err(|e| AppError::Internal(e.to_string()))?;
                attachments_map.insert(filename_str, base64::encode(content));
            }
            Some(attachments_map)
        } else {
            None
        };

        // Construct API-compatible Post
        Ok(Post {
            name: post_metadata.name,
            path: post_metadata.path,
            metadata: Metadata {
                id: Some(id.to_string()),
                extra: post_metadata.extra,
            },
            content,
            attachments,
        })
    }

    pub async fn delete(data_dir: &PathBuf, username: &str, id: &str) -> Result<()> {
        let user_dir = data_dir.join(username).join(id);
        if !user_dir.exists() {
            return Err(AppError::NotFound);
        }

        fs::remove_dir_all(user_dir).map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    pub fn render_content(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&self.content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    // Alias for load() to maintain compatibility
    pub async fn load_by_path(data_dir: &PathBuf, username: &str, id: &str) -> Result<Self> {
        Self::load(data_dir, username, id).await
    }
}
