use crate::system_info::AppSystemInfo;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn fixed_update(app_sys_fixed: &mut AppSystemInfo, app_sys_async: Arc<Mutex<AppSystemInfo>>) {
    match app_sys_async.try_lock() {
        Ok(app_sys_info_locked) => {
            app_sys_fixed.mem_usage = app_sys_info_locked.mem_usage;
            app_sys_fixed.swap_usage = app_sys_info_locked.swap_usage;
            app_sys_fixed.total_swap = app_sys_info_locked.total_swap;
            app_sys_fixed.load_average = app_sys_info_locked.load_average.clone();
            app_sys_fixed.networks = app_sys_info_locked.networks.clone();
            for i in 0..4 {
                match app_sys_fixed.cpu_usage_per_cpu.get_mut(i) {
                    Some(cpu_usage) => match app_sys_info_locked.cpu_usage_per_cpu.get(i) {
                        Some(updated_usage) => {
                            *cpu_usage = *updated_usage;
                        }
                        None => {}
                    },
                    None => {}
                }
            }
        }
        Err(_) => {}
    }
}
