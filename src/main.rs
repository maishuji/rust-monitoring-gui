use eframe::egui::{self, Frame, Stroke};
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Default)]
struct CpuMonitorApp {
    hostname: String,
    cpu_usage_fixed: f32,
    cpu_usage_per_cpu_fixed: Vec<f32>,
    mem_usage_fixed: f32,
    cpu_usage_per_cpu: Arc<Mutex<Vec<f32>>>, // Store the current CPU usage (shared, safe-thread)
    mem_usage: Arc<Mutex<f32>>,              // Store the current RAM usage (shared, safe-thread)
    system: Arc<Mutex<System>>,              // The system object to retrieve CPU usage
    cpu_count: usize,
    total_ram: u64,
}

impl CpuMonitorApp {}

async fn update_system_usage(
    system: Arc<Mutex<System>>,
    cpu_usage: Arc<Mutex<Vec<f32>>>,
    mem_usage: Arc<Mutex<f32>>,
) {
    // Update the CPU usage from the system stats

    loop {
        sleep(Duration::from_secs(1)).await;
        let mut tmp_cpu: Vec<f32> = vec![0.0; 4];
        let mut tmp_mem: f32 = 0.0;
        match system.try_lock() {
            Ok(mut system_locked) => {
                system_locked.refresh_cpu();

                for i in 0..4 {
                    tmp_cpu[i] = system_locked.cpus()[i].cpu_usage();
                }

                system_locked.refresh_all();
                let avail_mem = system_locked.available_memory();
                let total_mem = system_locked.total_memory();

                tmp_mem = (total_mem - avail_mem) as f32 / total_mem as f32 * 100.0;
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
                println!("Failed to acquire lock for cpu usage")
            }
        }
        match mem_usage.try_lock() {
            Ok(mut mem_usage_locked) => {
                *mem_usage_locked = tmp_mem;
            }
            Err(_) => {
                println!("Failed to acquire lock for mem usage")
            }
        }
    }
}

impl eframe::App for CpuMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let mlocked = self.cpu_usage.lock();
        match self.cpu_usage_per_cpu.try_lock() {
            Ok(cpu_usage_locked) => {
                for i in 0..self.cpu_count {
                    match self.cpu_usage_per_cpu_fixed.get_mut(i) {
                        Some(cpu_usage) => match cpu_usage_locked.get(i) {
                            Some(updated_usage) => {
                                *cpu_usage = *updated_usage;
                            }
                            None => {}
                        },
                        None => {}
                    }
                }

                match cpu_usage_locked.get(1) {
                    Some(cpu_usage) => {
                        self.cpu_usage_fixed = *cpu_usage;
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }

        match self.mem_usage.try_lock() {
            Ok(mem_usage_locked) => {
                self.mem_usage_fixed = *mem_usage_locked;
            }
            Err(_) => {}
        }

        // Show the central panel with CPU usage
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("System Monitoring");

            // CPU Information
            Frame::group(&ui.style())
                .stroke(Stroke::new(1.0, egui::Color32::BLACK)) // Set border thickness and color
                .rounding(5.0) // Optional: round the corners
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("CPU Count {}", self.cpu_count));
                        ui.label(format!("CPU Usage: {:.2}%", self.cpu_usage_fixed));
                    });

                    // Create a grid with 2 columns (for index and usage)
                    ui.label(format!("CPUS"));
                    egui::Grid::new("cpu_usage_grid")
                        .num_columns(2) // Number of columns
                        .show(ui, |ui| {
                            for (i, &usage) in self.cpu_usage_per_cpu_fixed.iter().enumerate() {
                                // Display CPU index and usage
                                ui.label(format!("CPU {}", i)); // CPU index (e.g., CPU 0)
                                ui.label(format!("{:.2}%", usage)); // CPU usage percentage
                                ui.end_row(); // End of the current row
                            }
                        });
                });

            // Memory Information
            Frame::group(&ui.style())
                .stroke(Stroke::new(1.0, egui::Color32::DARK_GRAY))
                .rounding(5.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "Total RAM (MB): {:.2}",
                            self.total_ram as f64 / 1024.0
                        ));
                        ui.label(format!("RAM Usage: {:.2}%", self.mem_usage_fixed));
                    });
                });

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
    let mut app = CpuMonitorApp::default();
    let system_shared = app.system.clone();
    let cpu_usage_shared = app.cpu_usage_per_cpu.clone();
    let mem_usage_shared = app.mem_usage.clone();

    match system_shared.try_lock() {
        Ok(mut system_lock) => {
            system_lock.refresh_all();
            println!("System name : {:?}", system_lock.host_name());
            app.cpu_count = system_lock.cpus().len();
            app.total_ram = system_lock.total_memory();
            app.cpu_usage_per_cpu_fixed = vec![0.0; app.cpu_count];

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
        update_system_usage(system_shared, cpu_usage_shared, mem_usage_shared).await;
        // Update CPU usage periodically
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
