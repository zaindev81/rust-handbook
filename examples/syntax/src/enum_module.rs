enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Debug => write!(f, "Debug"),
        }
    }
}

pub fn enum_main() {
    let level = LogLevel::Info;
    println!("Current log level: {}", level);
}