use crate::system_info::AppSystemInfo;
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct CpuMonitorApp {
    pub hostname: String,
    pub system: Arc<Mutex<System>>,
    pub app_sys_info: Arc<Mutex<AppSystemInfo>>,
    pub app_sys_info_fixed: AppSystemInfo,
    pub cpu_count: usize,
    pub os_version: String,
    pub kernel_version: String,
}

impl CpuMonitorApp {
    pub fn new(system: Arc<Mutex<System>>, app_sys_info: Arc<Mutex<AppSystemInfo>>) -> Self {
        CpuMonitorApp {
            system,
            app_sys_info,
            ..Default::default()
        }
    }
}
