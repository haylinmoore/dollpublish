use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Users {
    pub users: HashMap<String, User>,
    #[serde(skip)]
    data_dir: Option<PathBuf>,
}

impl Users {
    pub async fn load_or_create(data_dir: &PathBuf) -> Result<Arc<Mutex<Self>>> {
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).map_err(|e| AppError::Internal(e.to_string()))?;
        }

        let users_path = data_dir.join("users.json");
        if !users_path.exists() {
            let mut users = HashMap::new();
            users.insert(
                "default".to_string(),
                User {
                    api_key: Uuid::new_v4().to_string(),
                },
            );
            let users_data = Users {
                users,
                data_dir: Some(data_dir.clone()),
            };
            users_data.save().await?;

            return Ok(Arc::new(Mutex::new(users_data)));
        }

        let content =
            fs::read_to_string(&users_path).map_err(|e| AppError::Internal(e.to_string()))?;
        let mut users: Users =
            serde_json::from_str(&content).map_err(|e| AppError::Internal(e.to_string()))?;

        users.data_dir = Some(data_dir.clone());
        Ok(Arc::new(Mutex::new(users)))
    }

    async fn save(&self) -> Result<()> {
        let data_dir = self
            .data_dir
            .as_ref()
            .ok_or_else(|| AppError::Internal("data_dir not set".to_string()))?;

        fs::write(
            data_dir.join("users.json"),
            serde_json::to_string_pretty(&self).unwrap(),
        )
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn reload(&mut self) -> Result<()> {
        let data_dir = self
            .data_dir
            .as_ref()
            .ok_or_else(|| AppError::Internal("data_dir not set".to_string()))?;

        let content = fs::read_to_string(data_dir.join("users.json"))
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let reloaded: Users =
            serde_json::from_str(&content).map_err(|e| AppError::Internal(e.to_string()))?;

        self.users = reloaded.users;
        Ok(())
    }

    pub async fn verify_credentials(&mut self, api_key: &str, api_secret: &str) -> Option<String> {
        // First try with current data
        for (username, user) in self.users.iter() {
            if user.api_key == api_key || user.api_key == api_secret {
                return Some(username.clone());
            }
        }

        // If not found, reload and try again
        if let Ok(()) = self.reload().await {
            for (username, user) in self.users.iter() {
                if user.api_key == api_key || user.api_key == api_secret {
                    return Some(username.clone());
                }
            }
        }

        None
    }
}
