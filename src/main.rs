mod app;
mod cpu_monitor;
mod network_info;
mod system_info;
mod ui;
mod utils;

use eframe::egui::{self};
use sysinfo::SystemExt;

use app::CpuMonitorApp;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = CpuMonitorApp::default();
    let system_shared = app.system.clone();
    let app_sys_info_shared = app.app_sys_info.clone();

    match system_shared.try_lock() {
        Ok(mut system_lock) => {
            system_lock.refresh_all();
            println!("System name : {:?}", system_lock.host_name());
            app.cpu_count = system_lock.cpus().len();
            app.app_sys_info_fixed.total_mem = system_lock.total_memory();
            app.app_sys_info_fixed.cpu_usage_per_cpu = vec![0.0; app.cpu_count];
            app.os_version = system_lock
                .long_os_version()
                .unwrap_or(String::from("Unknown"));
            app.kernel_version = system_lock
                .kernel_version()
                .unwrap_or(String::from("Unknown"));

            match system_lock.host_name() {
                Some(hostname) => {
                    app.hostname = hostname;
                }
                None => {
                    app.hostname = String::from("<Unknown>");
                }
            }
        }
        Err(_) => {}
    }

    tokio::spawn(async move {
        // Update CPU usage periodically
        cpu_monitor::update_system_usage(system_shared, app_sys_info_shared).await;
    });

    // Run the native window with options
    eframe::run_native(
        "System Usage Monitor", // Window title
        eframe::NativeOptions {
            drag_and_drop_support: true,
            initial_window_size: Some(egui::vec2(400.0, 400.0)),
            ..Default::default()
        },
        Box::new(move |_cc| Box::new(app)),
    )
}
