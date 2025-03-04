use crate::network_info::NetworkInfo;
use crate::system_info::AppSystemInfo;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{CpuExt, LoadAvg, NetworkExt, System, SystemExt};
use tokio::sync::Mutex;
use tokio::time::sleep;

pub async fn update_system_usage(
    system: Arc<Mutex<System>>,
    app_sys_info: Arc<Mutex<AppSystemInfo>>,
) {
    loop {
        sleep(Duration::from_secs(1)).await;
        let mut tmp_cpu: Vec<f32> = vec![0.0; 4];
        let mut tmp_mem: f32 = 0.0;
        let mut tmp_swap: f32 = 0.0;
        let mut total_mem: u64 = 0;
        let mut total_swap: u64 = 0;
        let mut tmp_load_avg = LoadAvg::default();
        let mut tmp_networks: HashMap<String, NetworkInfo> = HashMap::new();

        match system.try_lock() {
            Ok(mut system_locked) => {
                system_locked.refresh_all();
                for i in 0..4 {
                    tmp_cpu[i] = system_locked.cpus()[i].cpu_usage();
                }
                tmp_load_avg = system_locked.load_average();
                let avail_mem = system_locked.available_memory();
                total_mem = system_locked.total_memory();
                total_swap = system_locked.total_swap();

                for ndata in system_locked.networks() {
                    let mut ninfo = NetworkInfo::default();
                    ninfo.rx = ndata.1.packets_received();
                    ninfo.tx = ndata.1.packets_transmitted();
                    tmp_networks.insert(ndata.0.clone(), ninfo.clone());
                }

                tmp_mem = (total_mem - avail_mem) as f32 / total_mem as f32 * 100.0;
                tmp_swap =
                    system_locked.used_swap() as f32 / system_locked.total_swap() as f32 * 100.0;
            }
            Err(_) => {
                println!("Failed to lock system")
            }
        }

        match app_sys_info.try_lock() {
            Ok(mut app_sys_info_locked) => {
                app_sys_info_locked.swap_usage = tmp_swap;
                app_sys_info_locked.cpu_count = tmp_cpu.len();
                app_sys_info_locked.total_mem = total_mem;
                app_sys_info_locked.mem_usage = tmp_mem;
                app_sys_info_locked.cpu_usage_per_cpu = tmp_cpu;
                app_sys_info_locked.total_swap = total_swap;
                app_sys_info_locked.load_average = tmp_load_avg.clone();
                app_sys_info_locked.networks = tmp_networks.clone();
            }
            Err(_) => {
                println!("Failed to acquire lock for swap usage")
            }
        }
    }
}
