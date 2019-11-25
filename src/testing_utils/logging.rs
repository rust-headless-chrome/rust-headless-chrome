use std::io::Write;

use chrono;
use env_logger;
use env_logger::fmt;
use log::*;

pub fn enable_logging() {
    let mut builder = env_logger::Builder::from_default_env();

    // NOTE: can infer types here, but I find them a useful reminder.
    let _result = builder
        .format(move |buf: &mut fmt::Formatter, record: &log::Record| {
            let date = chrono::Local::now();

            let level_str = level_to_emoji(record.level());
            let mut style = buf.style();
            let hours_minutes = date.format("%H:%M").to_string();
            let seconds_millis = date.format("%S%.3f").to_string();
            let fmt_seconds = style.set_bold(true).value(seconds_millis);

            let truncated_module_path = &record.module_path().unwrap()[5..];

            writeln!(
                buf,
                "{:<2} [{}:{}] - {:<12} - {}",
                level_str,
                hours_minutes,
                fmt_seconds,
                truncated_module_path,
                record.args()
            )
        })
        .try_init();
}

// damn, looks like it's harder than I thought it would be to change the datetime format!

fn level_to_emoji(level: log::Level) -> &'static str {
    use Level::*;

    match level {
        Error => "❌",
        Warn => "☢️  ",
        Info => "📝",
        Debug => "🐛", // NOTE: there's emoji here!
        Trace => "🏹",
    }
}

#[test]
fn start_the_logs() {
    enable_logging();

    error!("error message");
    warn!("warn message");
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
