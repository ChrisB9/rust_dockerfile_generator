use std::fmt::Display;
use terminal_color_builder::OutputFormatter as tcb;

pub fn success<T: Display>(t: T) -> () {
    println!(
        "{}",
        tcb::new()
            .fg()
            .hex("#fff")
            .bg()
            .green()
            .text(t.to_string())
            .print()
    );
}

pub fn error<T: Display>(t: T) -> () {
    println!(
        "{}",
        tcb::new()
            .fg()
            .hex("#fff")
            .bg()
            .red()
            .text(t.to_string())
            .print()
    )
}

pub fn cmd(str: &str) -> String {
    tcb::new().fg().hex("#6f0").text_str(str).print()
}
