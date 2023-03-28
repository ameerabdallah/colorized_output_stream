use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, AsRefStr};
use colored::{Colorize, ColoredString};

#[derive(Debug, EnumIter, EnumString, AsRefStr, Copy, Clone)]
#[strum(serialize_all = "UPPERCASE")]
enum LogLevel {
    Info,
    Warn,
    Debug,
    Trace,
    Error,
    Fatal,
    Panic,
    Critical
}

impl LogLevel {
    fn colorize(line: &str) -> ColoredString {

        let level = Self::parse(line.clone());
        
        match level {
            Some(LogLevel::Info) => line.cyan(),
            Some(LogLevel::Warn) => line.yellow(),
            Some(LogLevel::Debug) => line.blue(),
            Some(LogLevel::Trace) => line.magenta(),
            Some(LogLevel::Error) => line.red(),
            Some(LogLevel::Fatal) => line.red().bold(),
            Some(LogLevel::Panic) => line.red().bold(),
            Some(LogLevel::Critical) => line.red().bold(),
            None => line.normal(),

        }
    }

    fn parse(line: &str) -> Option<LogLevel> {
        let mut log_level_pair: Option<(LogLevel, usize)> = None; 

        for level in LogLevel::iter() {
            let level_string = level.as_ref();
            let level_index = line.find(level_string);
            if level_index.is_some() {
                if level_index.unwrap() == 0 {
                    return Some(level);
                } else if log_level_pair.is_none() || level_index.unwrap() < log_level_pair.unwrap().1 {
                    log_level_pair = Some((level, level_index.unwrap()));
                }
            }
        }

        log_level_pair.map_or(None, |pair| Some(pair.0))
    }
}


fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        println!("{}", LogLevel::colorize(buffer.as_ref()));
        buffer.clear();
    }
}



