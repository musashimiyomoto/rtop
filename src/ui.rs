use std::fmt::Write;

pub const C_RESET: &str = "\x1B[0m";
pub const C_CYAN: &str = "\x1B[36m";
pub const C_GREEN: &str = "\x1B[32m";
pub const C_YELLOW: &str = "\x1B[33m";
pub const C_MAGENTA: &str = "\x1B[35m";
pub const C_BLUE: &str = "\x1B[34m";
pub const C_RED: &str = "\x1B[31m";
pub const C_BOLD: &str = "\x1B[1m";

pub fn push_bar(buffer: &mut String, label: &str, percent: usize, _default_color: &str) {
    let width = 40;
    let percent = if percent > 100 { 100 } else { percent };
    
    let color_code = if percent < 50 {
        C_GREEN
    } else if percent < 80 {
        C_YELLOW
    } else {
        C_RED
    };

    let filled = (percent * width) / 100;
    let empty = width - filled;

    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);

    writeln!(buffer, " {:<7}: [ {}{}{}{} ] {:>3}%", 
        label, 
        color_code, 
        filled_str, 
        C_RESET, 
        empty_str, 
        percent
    ).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_bar() {
        let mut buffer = String::new();
        push_bar(&mut buffer, "TEST", 50, "");
        assert!(buffer.contains("TEST"));
        assert!(buffer.contains("["));
        assert!(buffer.contains("]"));
        assert!(buffer.contains("50%"));
    }
}
