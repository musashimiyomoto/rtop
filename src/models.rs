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
