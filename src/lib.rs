#![allow(dead_code)]

//https://www.youtube.com/watch?v=Uzyii85rwqo

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
//use std::fs::{File, OpenOptions};
//use std::io::Write;

#[derive(PartialEq, PartialOrd, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default] Info,
    Warn,
    Error,
}

#[derive(Default)]
struct LogEntry {
    level: LogLevel,
    message: String,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARNING",
            LogLevel::Error => "ERROR",
        };
        write!(f, "{}", level_str)
    }
}

pub struct Logger {
    session_start : SystemTime,
    log_level: LogLevel,
    use_subseconds_enabled : bool,
    subseconds_level : LogLevel,
    //file_output_enabled : bool,
   // output_file : Option<File>,
    
}
impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}
impl Logger {

    //Logger Constructor
    pub fn new() -> Self {
        let session_start=SystemTime::now();
        Self {
            session_start : session_start,
            log_level: LogLevel::Info,
            use_subseconds_enabled : true,
            subseconds_level : LogLevel::Debug,
            //file_output_enabled : true,
            //output_file : None,
        }
    }

    //Config Functions
    pub fn set_log_level(&mut self, log_level: LogLevel) { self.log_level = log_level; }

    pub fn set_subseconds_enabled(&mut self, enabled : bool){ self.use_subseconds_enabled = enabled; }

    pub fn set_subseconds_level(&mut self, level: LogLevel) { self.subseconds_level = level; }

    //fn set_file_output_enabled(&mut self, enabled : bool){ self.file_output_enabled = enabled; }

    //Main log function
    fn log(&self, log_entry: LogEntry) {

        //exit if filtered
        if log_entry.level < self.log_level {return;}

        //Get logging time
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let time_now_secs = time_now.as_secs();

        let hh = (time_now_secs/3600) %24;
        let mm = (time_now_secs%3600)/60;
        let ss = time_now_secs%60;

        if self.use_subseconds_enabled && (self.subseconds_level >= log_entry.level) {

            let time_now_subsecs = time_now.subsec_millis();
            println!("[{:02}:{:02}:{:02}.{:03}] [{}] {}", hh, mm, ss, time_now_subsecs, log_entry.level, log_entry.message);

        }else{
            println!("[{:02}:{:02}:{:02}] [{}] {}", hh, mm, ss, log_entry.level, log_entry.message);
        }
        
    }
    

    //Helper functions
    pub fn trace(&self, message : &str){
        self.log(LogEntry{level : LogLevel::Trace, message : message.to_string()})
    }

    pub fn debug(&self, message : &str){
        self.log(LogEntry{level : LogLevel::Debug, message : message.to_string()})
    }

    pub fn info(&self, message : &str){
        self.log(LogEntry{level : LogLevel::Info, message : message.to_string()})
    }

    pub fn warn(&self, message : &str){
        self.log(LogEntry{level : LogLevel::Warn, message : message.to_string()})
    }

    pub fn error(&self, message : &str){
        self.log(LogEntry{level : LogLevel::Error, message : message.to_string()})
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_helper_functions() {
        let mut logger = Logger::new();
        logger.set_log_level(LogLevel::Info);
        logger.set_subseconds_level(LogLevel::Info);
        logger.trace("trace message!");
        logger.debug("debug message!");
        logger.info("info message!");
        logger.warn("warn message!");
        logger.error("error message!");
    }
}
