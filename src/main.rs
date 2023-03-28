use std::io::Write;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, AsRefStr};
use termcolor::{Color, StandardStream, ColorChoice, WriteColor, ColorSpec};

#[derive(Debug, EnumIter, EnumString, AsRefStr, Copy, Clone)]
#[strum(serialize_all = "UPPERCASE")]
enum LogLevel {
    Info,
    Warn,
    Debug,
    Trace,
    Error,
    Fatal,
    Fail,
    Panic,
    Critical
}

impl LogLevel {
    fn get_color(line: &str) -> Option<Color> {

        let level = Self::parse(line.clone());
        
        match level {
            Some(LogLevel::Info) => Some(Color::Cyan),
            Some(LogLevel::Warn) => Some(Color::Yellow), 
            Some(LogLevel::Debug) => Some(Color::Blue),
            Some(LogLevel::Trace) => Some(Color::Magenta),
            Some(LogLevel::Error) => Some(Color::Red),
            Some(LogLevel::Fail) => Some(Color::Red),
            Some(LogLevel::Fatal) => Some(Color::Red),
            Some(LogLevel::Panic) => Some(Color::Red),
            Some(LogLevel::Critical) => Some(Color::Red),
            None => None,
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
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        let color = LogLevel::get_color(buffer.as_ref());
        stdout.set_color(ColorSpec::new().set_fg(color)).unwrap();
        write!(&mut stdout, "{}", buffer).unwrap();
        buffer.clear();
    }
}



