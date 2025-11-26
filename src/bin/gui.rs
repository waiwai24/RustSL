mod app;
mod config;
mod worker;
mod ui;

use eframe::egui;

fn main() {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 1200.0]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "RSL Builder",
        options,
        Box::new(|_cc| {
            Ok(Box::new(app::RSLApp::new()))
        }),
    );
}
