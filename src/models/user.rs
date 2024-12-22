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
            let users_data = Users { users };
            fs::write(
                &users_path,
                serde_json::to_string_pretty(&users_data).unwrap(),
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

            return Ok(Arc::new(Mutex::new(users_data)));
        }

        let content =
            fs::read_to_string(users_path).map_err(|e| AppError::Internal(e.to_string()))?;
        let users: Users =
            serde_json::from_str(&content).map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(Arc::new(Mutex::new(users)))
    }

    pub async fn verify_credentials(&self, api_key: &str, api_secret: &str) -> Option<String> {
        for (username, user) in self.users.iter() {
            if user.api_key == api_key || user.api_key == api_secret {
                return Some(username.clone());
            }
        }
        None
    }
}
