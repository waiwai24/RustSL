use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionItem {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub encrypt_arg: String,
    pub feature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocMemMode {
    pub id: String,
    pub label: String,
    pub feature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMode {
    pub id: String,
    pub label: String,
    pub feature: String,
    #[serde(default)]
    pub pattern: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMCheck {
    pub id: String,
    pub label: String,
    pub feature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default)]
    pub encryption: Option<String>,
    #[serde(default)]
    pub run_mode: Option<String>,
    #[serde(default)]
    pub alloc_mem_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsManifest {
    pub encryption: Vec<EncryptionItem>,
    pub alloc_mem_modes: Vec<AllocMemMode>,
    pub run_modes: Vec<RunMode>,
    pub vm_checks: Vec<VMCheck>,
    #[serde(default)]
    pub defaults: Defaults,
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load_manifest() -> Result<PluginsManifest> {
        let path = Path::new("config/plugins.json");
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config/plugins.json: {}", e))?;
        
        let manifest: PluginsManifest = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse config/plugins.json: {}", e))?;
        
        if manifest.encryption.is_empty() || manifest.run_modes.is_empty() {
            return Err(anyhow!("Missing required fields in plugins.json"));
        }
        
        Ok(manifest)
    }
    
    pub fn get_encryption_map(manifest: &PluginsManifest) -> HashMap<String, String> {
        manifest
            .encryption
            .iter()
            .map(|e| {
                let arg = if e.encrypt_arg.is_empty() {
                    e.id.clone()
                } else {
                    e.encrypt_arg.clone()
                };
                (e.id.clone(), arg)
            })
            .collect()
    }
    
    pub fn get_encryption_feature_map(manifest: &PluginsManifest) -> HashMap<String, String> {
        manifest
            .encryption
            .iter()
            .map(|e| (e.id.clone(), e.feature.clone()))
            .collect()
    }
    
    pub fn get_vm_checks_map(manifest: &PluginsManifest) -> HashMap<String, String> {
        manifest
            .vm_checks
            .iter()
            .map(|v| (v.id.clone(), v.feature.clone()))
            .collect()
    }
    
    pub fn get_run_mode_map(manifest: &PluginsManifest) -> HashMap<String, String> {
        manifest
            .run_modes
            .iter()
            .map(|r| (r.id.clone(), r.feature.clone()))
            .collect()
    }
    
    pub fn get_alloc_mem_feature_map(manifest: &PluginsManifest) -> HashMap<String, String> {
        manifest
            .alloc_mem_modes
            .iter()
            .map(|m| (m.id.clone(), m.feature.clone()))
            .collect()
    }
    
    pub fn get_default_value(manifest: &PluginsManifest, key: &str) -> Option<String> {
        match key {
            "encryption" => manifest.defaults.encryption.clone(),
            "run_mode" => manifest.defaults.run_mode.clone(),
            "alloc_mem_mode" => manifest.defaults.alloc_mem_mode.clone(),
            _ => None,
        }
    }
}
