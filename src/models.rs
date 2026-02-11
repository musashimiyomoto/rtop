pub struct SystemInfo {
    pub os_name: String,
    pub host_name: String,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
}
