#![allow(dead_code)]
use crate::utilities::files::get_user_cache_file;
use amethyst::config::Config;
use log::error;
use serde::{Deserialize, Serialize};

/// These are some transient values to improve user experience.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserCache {
    /// todo: unimplemented
    pub name: String,
}

impl UserCache {
    pub fn save_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.write(get_user_cache_file()).unwrap_or_else(|err| {
            error!("Failed to save {:?} because error: {:?}", self, err);
        });
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
