use crate::network_info::NetworkInfo;
use std::collections::HashMap;
use sysinfo::LoadAvg;

#[derive(Default)]
pub struct AppSystemInfo {
    pub cpu_count: usize,
    pub total_mem: u64,
    pub mem_usage: f32,
    pub total_swap: u64,
    pub swap_usage: f32,
    pub cpu_usage_per_cpu: Vec<f32>,
    pub load_average: LoadAvg,
    pub networks: HashMap<String, NetworkInfo>,
}
