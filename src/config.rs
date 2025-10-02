use anyhow::Result;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, RoleId};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub category_id: u64,
    pub log_channel_id: u64,
    pub list_channel_id: u64,
    pub special_role_id: u64,
    pub manager_role_id: u64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn category_id(&self) -> ChannelId {
        ChannelId::new(self.category_id)
    }

    pub fn log_channel_id(&self) -> ChannelId {
        ChannelId::new(self.log_channel_id)
    }

    pub fn list_channel_id(&self) -> ChannelId {
        ChannelId::new(self.list_channel_id)
    }

    pub fn special_role_id(&self) -> RoleId {
        RoleId::new(self.special_role_id)
    }

    pub fn manager_role_id(&self) -> RoleId {
        RoleId::new(self.manager_role_id)
    }
}