use console::{style, Color, Term};

#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
}

pub struct Logger {
    pub level: LogLevel,
    pub out: Term,
    pub err: Term,
}

impl LogLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Trace => "Trace",
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
        }
    }
}

impl Logger {
    pub fn new(level: LogLevel) -> Logger {
        Logger {
            level,
            out: Term::stdout(),
            err: Term::stderr(),
        }
    }
    pub fn log_trace(&self, message: &str) -> std::io::Result<()> {
        self.log(message, LogLevel::Trace)
    }

    pub fn log_debug(&self, message: &str) -> std::io::Result<()> {
        self.log(message, LogLevel::Debug)
    }

    pub fn log_info(&self, message: &str) -> std::io::Result<()> {
        self.log(message, LogLevel::Info)
    }

    pub fn log_warning(&self, message: &str) -> std::io::Result<()> {
        self.log(message, LogLevel::Warning)
    }

    pub fn log_error(&self, message: &str) -> std::io::Result<()> {
        self.log(message, LogLevel::Error)
    }

    pub fn log(&self, message: &str, level: LogLevel) -> std::io::Result<()> {
        if (level.clone() as u8) < (self.level.to_owned() as u8) {
            return Ok(());
        }

        let mut std = &self.out;

        if level == LogLevel::Error {
            std = &self.err;
        }

        std.write_line(&*format!(
            "[{}] {}",
            style(level.to_string()).fg(match level {
                LogLevel::Trace => Color::White,
                LogLevel::Debug => Color::Blue,
                LogLevel::Info => Color::Green,
                LogLevel::Warning => Color::Yellow,
                LogLevel::Error => Color::Red,
            }),
            message
        ))
    }
}
