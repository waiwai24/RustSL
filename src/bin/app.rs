use eframe::egui;
use std::sync::mpsc::Receiver;
use std::path::PathBuf;
use crate::config::{ConfigManager, PluginsManifest};
use crate::worker::{BuildWorker, BuildParams, WorkerMessage};
#[allow(unused_imports)]
use crate::ui;

pub struct RSLApp {
    manifest: PluginsManifest,
    
    // File pickers
    bin_file: Option<PathBuf>,
    bin_display: String,
    ico_file: Option<PathBuf>,
    ico_display: String,
    sign_app_file: Option<PathBuf>,
    sign_app_display: String,
    
    // Combo boxes (store as indices and items)
    encryption_idx: usize,
    encryption_items: Vec<(String, String)>,
    encode_idx: usize,
    encode_items: Vec<(String, String)>,
    mem_mode_idx: usize,
    mem_mode_items: Vec<(String, String)>,
    run_mode_idx: usize,
    run_mode_items: Vec<(String, String)>,
    target_idx: usize,
    target_items: Vec<(String, String)>,
    
    // Text inputs
    target_program_input: String,
    target_pid_input: String,
    
    // Checkboxes
    vm_checks: Vec<(String, String, bool)>, // (id, label, checked)
    sign_enable: bool,
    forgery_enable: bool,
    win7_compat: bool,
    
    // Build state
    building: bool,
    log_output: String,
    progress: u32,
    worker_rx: Option<Receiver<WorkerMessage>>,
    
    // UI state
    show_target_input: bool,
    show_pid_input: bool,
}

impl Default for RSLApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RSLApp {
    pub fn new() -> Self {
        let manifest = ConfigManager::load_manifest()
            .expect("Failed to load config/plugins.json");
        
        let encryption_items = manifest.encryption
            .iter()
            .map(|e| (e.label.clone(), e.id.clone()))
            .collect::<Vec<_>>();
        
        let mut encryption_idx = 0;
        if let Some(default) = ConfigManager::get_default_value(&manifest, "encryption") {
            for (idx, (_, id)) in encryption_items.iter().enumerate() {
                if id == &default {
                    encryption_idx = idx;
                    break;
                }
            }
        }
        
        let encode_items = vec![
            ("base64".to_string(), "base64".to_string()),
            ("base32".to_string(), "base32".to_string()),
            ("none".to_string(), "none".to_string()),
        ];
        
        let mem_mode_items = manifest.alloc_mem_modes
            .iter()
            .map(|m| (m.label.clone(), m.id.clone()))
            .collect::<Vec<_>>();
        
        let mut mem_mode_idx = 0;
        if let Some(default) = ConfigManager::get_default_value(&manifest, "alloc_mem_mode") {
            for (idx, (_, id)) in mem_mode_items.iter().enumerate() {
                if id == &default {
                    mem_mode_idx = idx;
                    break;
                }
            }
        }
        
        let run_mode_items = manifest.run_modes
            .iter()
            .map(|r| (r.label.clone(), r.id.clone()))
            .collect::<Vec<_>>();
        
        let mut run_mode_idx = 0;
        if let Some(default) = ConfigManager::get_default_value(&manifest, "run_mode") {
            for (idx, (_, id)) in run_mode_items.iter().enumerate() {
                if id == &default {
                    run_mode_idx = idx;
                    break;
                }
            }
        }
        
        let target_items = vec![
            ("Windows MSVC (x64)".to_string(), "x86_64-pc-windows-msvc".to_string()),
            ("Windows MSVC (x86)".to_string(), "i686-pc-windows-msvc".to_string()),
            ("Windows GNU (x64)".to_string(), "x86_64-pc-windows-gnu".to_string()),
            ("Windows GNU (x86)".to_string(), "i686-pc-windows-gnu".to_string()),
            ("Windows MSVC (ARM64)".to_string(), "aarch64-pc-windows-msvc".to_string()),
        ];
        
        let vm_checks = manifest.vm_checks
            .iter()
            .map(|v| (v.id.clone(), v.label.clone(), false))
            .collect::<Vec<_>>();
        
        RSLApp {
            manifest,
            bin_file: None,
            bin_display: "No file selected".to_string(),
            ico_file: None,
            ico_display: "icons/excel.ico".to_string(),
            sign_app_file: None,
            sign_app_display: "No app selected".to_string(),
            encryption_idx,
            encryption_items,
            encode_idx: 0,
            encode_items,
            mem_mode_idx,
            mem_mode_items,
            run_mode_idx,
            run_mode_items,
            target_idx: 0,
            target_items,
            target_program_input: r"C:/Windows/System32/werfault.exe".to_string(),
            target_pid_input: "0".to_string(),
            vm_checks,
            sign_enable: false,
            forgery_enable: false,
            win7_compat: false,
            building: false,
            log_output: String::new(),
            progress: 0,
            worker_rx: None,
            show_target_input: false,
            show_pid_input: false,
        }
    }
    
    fn collect_params(&self) -> BuildParams {
        let input_bin = self.bin_file
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("calc.bin")
            .to_string();
        
        let run_mode = self.run_mode_items
            .get(self.run_mode_idx)
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "enum_ui".to_string());
        
        let vm_checks = self.vm_checks
            .iter()
            .filter(|(_, _, checked)| *checked)
            .map(|(id, _, _)| id.clone())
            .collect::<Vec<_>>()
            .join(",");
        
        let enc_method = self.encryption_items
            .get(self.encryption_idx)
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "ipv4".to_string());
        
        let encode_method = self.encode_items
            .get(self.encode_idx)
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "base64".to_string());
        
        let icon_path = self.ico_file
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("icons/excel.ico")
            .to_string();
        
        let mem_mode = self.mem_mode_items
            .get(self.mem_mode_idx)
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "alloc_mem_va".to_string());
        
        let target = self.target_items
            .get(self.target_idx)
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "x86_64-pc-windows-msvc".to_string());
        
        BuildParams {
            input_bin,
            run_mode,
            vm_checks,
            enc_method,
            encode_method,
            icon_path,
            sign_enable: self.sign_enable,
            sign_app: self.sign_app_file.as_ref().and_then(|p| p.to_str()).map(|s| s.to_string()),
            forgery_enable: self.forgery_enable,
            mem_mode,
            target,
            target_program: self.target_program_input.clone(),
            target_pid: self.target_pid_input.clone(),
            win7_compat: self.win7_compat,
        }
    }
    
    fn start_build(&mut self) {
        if self.building {
            return;
        }
        
        self.building = true;
        self.log_output.clear();
        self.progress = 0;
        
        let params = self.collect_params();
        let worker = BuildWorker::new(self.manifest.clone());
        let rx = worker.start_build(params);
        self.worker_rx = Some(rx);
    }
    
    fn process_worker_messages(&mut self) {
        if let Some(ref rx) = self.worker_rx {
            loop {
                match rx.try_recv() {
                    Ok(WorkerMessage::Log(text)) => {
                        self.log_output.push_str(&text);
                        self.log_output.push('\n');
                    }
                    Ok(WorkerMessage::Progress(p)) => {
                        self.progress = p;
                    }
                    Ok(WorkerMessage::Done(path)) => {
                        self.log_output.push_str(&format!("Success: {}\n", path));
                        self.building = false;
                        self.worker_rx = None;
                        break;
                    }
                    Ok(WorkerMessage::Error(err)) => {
                        self.log_output.push_str(&format!("Error: {}\n", err));
                        self.building = false;
                        self.worker_rx = None;
                        break;
                    }
                    Err(_) => break,
                }
            }
        }
    }
    
    fn on_run_mode_changed(&mut self) {
        let pattern = self.run_mode_items
            .get(self.run_mode_idx)
            .and_then(|(_, id)| {
                self.manifest.run_modes
                    .iter()
                    .find(|r| r.id == *id)
                    .map(|r| r.pattern)
            })
            .unwrap_or(1);
        
        self.show_target_input = pattern == 2;
        self.show_pid_input = pattern == 3;
    }
}

impl eframe::App for RSLApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_worker_messages();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("🚀 RSL Builder");
                ui.separator();
                
                // 1. Shellcode Input
                ui.group(|ui| {
                    ui.label("📦 Shellcode");
                    ui.horizontal(|ui| {
                        ui.label(&self.bin_display);
                        if ui.button("📁 Select").clicked() {
                            if let Ok(Some(path)) = rfd::FileDialog::new()
                                .add_filter("binary", &["bin"])
                                .pick_file()
                            {
                                self.bin_display = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                self.bin_file = Some(path);
                            }
                        }
                    });
                });
                
                // 2. Encryption Method
                ui.group(|ui| {
                    ui.label("🔒 Encryption Method");
                    ui.horizontal(|ui| {
                        ui.label("Encryption:");
                        let enc_label = self.encryption_items.get(self.encryption_idx)
                            .map(|(l, _)| l.as_str())
                            .unwrap_or("Select");
                        
                        egui::ComboBox::from_label("")
                            .selected_text(enc_label)
                            .show_ui(ui, |ui| {
                                for (idx, (label, _)) in self.encryption_items.iter().enumerate() {
                                    ui.selectable_value(&mut self.encryption_idx, idx, label);
                                }
                            });
                        
                        ui.label("Encoding:");
                        let encode_label = self.encode_items.get(self.encode_idx)
                            .map(|(l, _)| l.as_str())
                            .unwrap_or("Select");
                        
                        egui::ComboBox::from_label("")
                            .selected_text(encode_label)
                            .show_ui(ui, |ui| {
                                for (idx, (label, _)) in self.encode_items.iter().enumerate() {
                                    ui.selectable_value(&mut self.encode_idx, idx, label);
                                }
                            });
                    });
                });
                
                // 3. Icon Selection
                ui.group(|ui| {
                    ui.label("🎨 Icon File");
                    ui.horizontal(|ui| {
                        ui.label(&self.ico_display);
                        if ui.button("📁 Select").clicked() {
                            if let Ok(Some(path)) = rfd::FileDialog::new()
                                .add_filter("icon", &["ico"])
                                .pick_file()
                            {
                                self.ico_display = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                self.ico_file = Some(path);
                            }
                        }
                    });
                });
                
                // 4. Memory Allocation Mode
                ui.group(|ui| {
                    ui.label("💾 Memory Allocation Mode");
                    let mem_label = self.mem_mode_items.get(self.mem_mode_idx)
                        .map(|(l, _)| l.as_str())
                        .unwrap_or("Select");
                    
                    egui::ComboBox::from_label("")
                        .selected_text(mem_label)
                        .show_ui(ui, |ui| {
                            for (idx, (label, _)) in self.mem_mode_items.iter().enumerate() {
                                ui.selectable_value(&mut self.mem_mode_idx, idx, label);
                            }
                        });
                });
                
                // 5. VM Checks
                ui.group(|ui| {
                    ui.label("🔍 Sandbox Detection");
                    let mut idx = 0;
                    while idx < self.vm_checks.len() {
                        ui.horizontal(|ui| {
                            let (_, label, checked) = &mut self.vm_checks[idx];
                            ui.checkbox(checked, label);
                            
                            if idx + 1 < self.vm_checks.len() {
                                let (_, label2, checked2) = &mut self.vm_checks[idx + 1];
                                ui.checkbox(checked2, label2);
                            }
                        });
                        idx += 2;
                    }
                });
                
                // 6. Run Mode
                ui.group(|ui| {
                    ui.label("⚙️ Run Mode");
                    let run_label = self.run_mode_items.get(self.run_mode_idx)
                        .map(|(l, _)| l.as_str())
                        .unwrap_or("Select");
                    
                    let prev_idx = self.run_mode_idx;
                    egui::ComboBox::from_label("")
                        .selected_text(run_label)
                        .show_ui(ui, |ui| {
                            for (idx, (label, _)) in self.run_mode_items.iter().enumerate() {
                                ui.selectable_value(&mut self.run_mode_idx, idx, label);
                            }
                        });
                    
                    if prev_idx != self.run_mode_idx {
                        self.on_run_mode_changed();
                    }
                    
                    if self.show_target_input {
                        ui.label("Target Program:");
                        ui.text_edit_singleline(&mut self.target_program_input);
                    }
                    if self.show_pid_input {
                        ui.label("Target Process ID:");
                        ui.text_edit_singleline(&mut self.target_pid_input);
                    }
                });
                
                // 7. Signature Forgery
                ui.group(|ui| {
                    ui.label("✍️ Signature Forgery");
                    ui.horizontal(|ui| {
                        ui.label(&self.sign_app_display);
                        if ui.button("📁 Select").clicked() {
                            if let Ok(Some(path)) = rfd::FileDialog::new()
                                .add_filter("executable", &["exe"])
                                .pick_file()
                            {
                                self.sign_app_display = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                self.sign_app_file = Some(path);
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.sign_enable, "Enable Signature");
                        ui.checkbox(&mut self.forgery_enable, "File Bundling");
                    });
                });
                
                // 8. Progress Bar
                ui.horizontal(|ui| {
                    ui.label("Progress:");
                    ui.add(egui::ProgressBar::new(self.progress as f32 / 100.0).show_percentage());
                });
                
                ui.separator();
                
                // 9. Build Configuration and Log
                ui.horizontal(|ui| {
                    // Left: Log output
                    ui.vertical(|ui| {
                        ui.label("📋 Build Log");
                        egui::ScrollArea::vertical()
                            .show(ui, |ui| {
                                ui.text_edit_multiline(&mut self.log_output);
                            });
                    });
                    
                    // Right: Build settings and button
                    ui.vertical(|ui| {
                        ui.label("📍 Build Target");
                        let target_label = self.target_items.get(self.target_idx)
                            .map(|(l, _)| l.as_str())
                            .unwrap_or("Select");
                        
                        egui::ComboBox::from_label("")
                            .selected_text(target_label)
                            .show_ui(ui, |ui| {
                                for (idx, (label, _)) in self.target_items.iter().enumerate() {
                                    ui.selectable_value(&mut self.target_idx, idx, label);
                                }
                            });
                        
                        ui.checkbox(&mut self.win7_compat, "Win7 Compatibility");
                        
                        ui.separator();
                        
                        let button_text = if self.building { "⏳ Building..." } else { "🚀 Generate" };
                        if ui.button(button_text).clicked() && !self.building {
                            self.start_build();
                        }
                    });
                });
            });
        });
        
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
