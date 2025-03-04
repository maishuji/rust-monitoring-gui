use crate::app::CpuMonitorApp;
use eframe::egui::{self, Frame, Stroke};
use sysinfo::SystemExt;

impl eframe::App for CpuMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::utils::fixed_update(&mut self.app_sys_info_fixed, self.app_sys_info.clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("System Monitoring");

            Frame::group(&ui.style())
                .stroke(Stroke::new(1.0, egui::Color32::BLACK))
                .rounding(5.0)
                .show(ui, |ui| {
                    Frame::group(&ui.style())
                        .stroke(Stroke::new(1.0, egui::Color32::DARK_GRAY))
                        .rounding(5.0)
                        .show(ui, |ui| {
                            ui.label(format!("OS Version:  {}", self.os_version));
                            ui.label(format!("Kernel: {}", self.kernel_version));
                            ui.separator();
                            ui.label(format!("CPU Count {}", self.cpu_count));
                        });

                    ui.vertical(|ui| {
                        ui.heading("Networks");
                        if self.app_sys_info_fixed.networks.len() == 0 {
                            ui.label(String::from("No information available"));
                        }
                        for (net_name, net_info) in &self.app_sys_info_fixed.networks {
                            ui.label(format!(
                                "Name:{}, tx:{},rx:{}",
                                net_name, net_info.tx, net_info.rx
                            ));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let load_avg = &self.app_sys_info_fixed.load_average;
                            ui.heading("Load Average");
                            ui.label(format!("15min.: {}", load_avg.fifteen));
                            ui.label(format!(" 5min.: {}", load_avg.five));
                            ui.label(format!(" 1min.: {}", load_avg.one));
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.heading(format!("CPUS Usage  [total - available]"));
                            egui::Grid::new("cpu_usage_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    for (i, &usage) in
                                        self.app_sys_info_fixed.cpu_usage_per_cpu.iter().enumerate()
                                    {
                                        ui.label(format!("CPU {}", i));
                                        ui.label(format!("{:.2}%", usage));
                                        ui.end_row();
                                    }
                                });
                        });
                    });
                });

            Frame::group(&ui.style())
                .stroke(Stroke::new(1.0, egui::Color32::DARK_GRAY))
                .rounding(5.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "Total RAM (MB): {:.2}",
                            self.app_sys_info_fixed.total_mem as f64 / 1024.0
                        ));
                        ui.label(format!(
                            "RAM Usage: {:.2}%",
                            self.app_sys_info_fixed.mem_usage
                        ));
                    });
                });
            Frame::group(&ui.style())
                .stroke(Stroke::new(1.0, egui::Color32::DARK_GRAY))
                .rounding(5.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "Total Swap (MB): {:.2}",
                            self.app_sys_info_fixed.total_swap as f64 / 1024.0
                        ));
                    });
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "Swap usage: {:.2}%",
                            self.app_sys_info_fixed.swap_usage
                        ));
                    });
                });
            ctx.request_repaint();
        });
    }

    fn on_exit(&mut self, _ctx: &eframe::glow::Context) {
        match self.system.try_lock() {
            Ok(mut system_locked) => {
                system_locked.refresh_all();
            }
            Err(_) => {}
        }
    }
}
