use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use rtop::models::DashboardData;
use rtop::sys::{get_system_info, get_uptime, SysCollector};
use rtop::ui::render_dashboard;

fn main() -> io::Result<()> {
    print!("\x1B[?1049h\x1B[?25l\x1B[2J");
    let mut stdout = io::stdout();

    let sys_info = get_system_info();
    let mut collector = SysCollector::new();

    loop {
        collector.refresh();

        let (mem_used, mem_total, mem_pct) = collector.memory_usage();
        let top_procs = collector.top_processes(5);

        let data = DashboardData {
            host_name: &sys_info.host_name,
            os_name: &sys_info.os_name,
            proc_count: collector.process_count(),
            uptime: get_uptime(),
            cpu: collector.cpu_load(),
            mem_used,
            mem_total,
            mem_pct,
            top_procs: &top_procs,
        };

        let mut buffer = String::new();
        buffer.push_str("\x1B[2J\x1B[H");
        render_dashboard(&mut buffer, &data);

        print!("{}", buffer);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(2000));
    }
}
