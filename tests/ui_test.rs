use rtop::ui::{push_bar, format_uptime, render_dashboard};
use rtop::models::ProcessInfo;

#[test]
fn test_push_bar() {
    let mut buffer = String::new();
    push_bar(&mut buffer, "TEST", 50, "");
    assert!(buffer.contains("TEST"));
    assert!(buffer.contains("["));
    assert!(buffer.contains("]"));
    assert!(buffer.contains("50%"));
}

#[test]
fn test_push_bar_clamps_over_100() {
    let mut buffer = String::new();
    push_bar(&mut buffer, "X", 150, "");
    assert!(buffer.contains("100%"));
}

#[test]
fn test_format_uptime() {
    assert_eq!(format_uptime(90), "1m 30s");
    assert_eq!(format_uptime(3661), "1h 1m 1s");
    assert_eq!(format_uptime(90000), "1d 1h 0m");
}

#[test]
fn test_render_dashboard() {
    let procs = vec![
        ProcessInfo { pid: 1, name: "test".to_string(), cpu_usage: 10.0, memory_mb: 50.0 },
    ];
    let mut buffer = String::new();
    render_dashboard(&mut buffer, "myhost", "Linux", 42, 3600, 25, 2.0, 8.0, 25, &procs);
    assert!(buffer.contains("RUST TOP DASHBOARD"));
    assert!(buffer.contains("myhost"));
    assert!(buffer.contains("Linux"));
    assert!(buffer.contains("test"));
}
