use eframe::egui;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Default)]
struct CpuMonitorApp {
    cpu_usage_fixed: f32,
    cpu_usage: Arc<Mutex<f32>>, // Store the current CPU usage (shared, safe-thread)
    system: Arc<Mutex<System>>, // The system object to retrieve CPU usage
}

impl CpuMonitorApp {}

async fn update_cpu_usage(system: Arc<Mutex<System>>, cpu_usage: Arc<Mutex<f32>>) {
    // Update the CPU usage from the system stats

    loop {
        sleep(Duration::from_secs(1)).await;
        let mut tmp_cpu: f32 = 0.0;
        match system.try_lock() {
            Ok(mut system_locked) => {
                system_locked.refresh_cpu();
                if let Some(cpu) = system_locked.cpus().get(0) {
                    tmp_cpu = cpu.cpu_usage();
                    // println!(" TmpCPU : {}", tmp_cpu);
                }
            }
            Err(_) => {
                println!("Failed to lock system")
            }
        }

        match cpu_usage.try_lock() {
            Ok(mut cpu_usage_locked) => {
                *cpu_usage_locked = tmp_cpu;
            }
            Err(_) => {
                println!("Failed to acquire lock in the spawned thread")
            }
        }
    }
}

impl eframe::App for CpuMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let mlocked = self.cpu_usage.lock();
        match self.cpu_usage.try_lock() {
            Ok(cpu_usage_locked) => {
                self.cpu_usage_fixed = *cpu_usage_locked;
            }
            Err(_) => {}
        }

        // Show the central panel with CPU usage
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CPU Usage Monitor");
            ui.label(format!("CPU Usage: {:.2}%", self.cpu_usage_fixed));
            // Update every 1 second
            ctx.request_repaint();
        });
    }

    fn on_exit(&mut self, _ctx: &eframe::glow::Context) {
        // Make sure to clean up the system resource
        match self.system.try_lock() {
            Ok(mut system_locked) => {
                system_locked.refresh_all();
            }
            Err(_) => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = CpuMonitorApp::default();
    let system_shared = app.system.clone();
    let cpu_usage_shared = app.cpu_usage.clone();
    tokio::spawn(async move {
        update_cpu_usage(system_shared, cpu_usage_shared).await; // Update CPU usage periodically
    });

    // Run the native window with options
    eframe::run_native(
        "CPU Usage Monitor", // Window title
        eframe::NativeOptions {
            drag_and_drop_support: true,
            initial_window_size: Some(egui::vec2(400.0, 200.0)),
            ..Default::default()
        },
        Box::new(move |_cc| Box::new(app)),
    )
}
