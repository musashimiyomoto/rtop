use rtop::sys::{get_system_info, SysCollector};

#[test]
fn test_system_info() {
    let info = get_system_info();
    assert!(!info.os_name.is_empty());
}

#[test]
fn test_collector_memory() {
    let collector = SysCollector::new();
    let (used, total, pct) = collector.memory_usage();
    assert!(total >= 0.0);
    assert!(used <= total);
    assert!(pct <= 100);
}

#[test]
fn test_collector_processes() {
    let collector = SysCollector::new();
    assert!(collector.process_count() > 0);

    let top = collector.top_processes(3);
    assert!(!top.is_empty());
    assert!(top.len() <= 3);
}
