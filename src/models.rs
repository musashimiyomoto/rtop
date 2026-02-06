pub struct SystemInfo {
    pub os_name: String,
    pub host_name: String,
}

pub struct DiskInfo {
    pub name: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub percent: u64,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
}

pub struct NetworkInfo {
    pub name: String,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
}
