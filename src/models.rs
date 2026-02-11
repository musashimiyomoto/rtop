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

pub struct DashboardData<'a> {
    pub host_name: &'a str,
    pub os_name: &'a str,
    pub proc_count: usize,
    pub uptime: u64,
    pub cpu: u64,
    pub mem_used: f64,
    pub mem_total: f64,
    pub mem_pct: u64,
    pub top_procs: &'a [ProcessInfo],
}
