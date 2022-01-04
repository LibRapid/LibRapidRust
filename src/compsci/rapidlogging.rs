//! Basic logging functionality.
use chrono::{Utc, SecondsFormat};
use std::io::prelude::*;
use std::fs::OpenOptions;

use crate::math::general::Increment;

/// The structure for the RapidLogger.
pub struct Logger {
    /// The buffer size. If the buffer count exceeds this value, the buffer gets written to the specified file.
    pub buff_size:      usize,
        buff_count:     usize,
        buffer:         String,
    /// Determines wether to write to the console.
    pub log_to_console: bool,
    /// Determines wether to write to a file.
    pub log_to_file:    bool,
    /// The optional file path.
    pub file_path:      Option<String>,
}

impl Logger {
    /// Creates a new `Logger` object.
    pub fn new(buff_size:      usize,
               log_to_console: bool,
               log_to_file:    bool,
               file_path:      Option<String>)
    -> Logger {
                Logger { buff_size,
                         buffer:     String::new(),
                         buff_count: 0,
                         log_to_console,
                         log_to_file,
                         file_path }
    }
    /// Creates a new `Logger` object with default values.
    pub fn new_default() -> Logger {
        Logger { buff_size:      10,
                 buff_count:     0,
                 buffer:         String::new(),
                 log_to_console: true,
                 log_to_file:    false,
                 file_path:      None }
    }
    /// Logs to a `Logger`.
    /// # Returns
    /// A `Result<bool, String>`. `true` if it was successful, otherwise the error message as a `String`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::rapidlogging::Logger;
    /// let mut l: Logger = Logger::new(3, true, true, Some("log.txt".to_string()));
    /// let _ = l.log(None, "Test-Log.");
    /// let _ = l.log(None, "Test-Log.");
    /// let _ = l.log(None, "Test-Log.");
    /// ```
    /// As you can see, we initialise a new Logger `l` with the buffer size 3. This means that only after 3x logging, the logger writes to the file.
    /// Instead of `None`, you can also add things like `some(vec!["warning", "low urgency"])`. Then, one line of the output will look like this:
    /// `[2021-12-13T18:08:06Z][warning][low urgency] Test-Log.`
    pub fn log(&mut self, prefixes: Option<Vec<&str>>, msg: &str) -> Result<bool, String> {
        self.buff_count.inc();
        let mut out: String = format!("[{}]", Utc::now()
                                                   .to_rfc3339_opts(SecondsFormat::Secs,
                                                                    true));
        match prefixes {
            Some(v) => {
                for s in v {
                    out.push_str(&format!("[{}]", s));
                }
            }
            None => { }
        }
        out.push(' ');
        out.push_str(msg);
        out.push('\n');
        self.buffer += &out;

        if self.buff_count == self.buff_size {
            return self.backend_log();
        }
        Ok(true)
    }
    /// Resets `buffer` and `buff_counter`.
    pub fn reset_buffs(&mut self) {
        self.buff_count = 0;
        self.buffer = String::new();
    }

    fn backend_log(&mut self) -> Result<bool, String> {
        if self.log_to_file { 
            let file = OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(&self.file_path.as_ref().unwrap());
            let mut file = match file {
                Ok(f)  => f,
                Err(e) => { return Err(format!("Problem opening or creating file: {:?}", e)); }
            };
            match write!(file, "{}", self.buffer) {
                Ok(_)  => { }
                Err(e) => { return Err(format!("Problem writing to file: {:?}", e)); }
            }
        }

        if self.log_to_console {
            print!("{}", self.buffer);
            std::io::stdout().flush().unwrap();
        }
        self.reset_buffs();
        Ok(true)
    }
}

#[test]
fn test() {
    let mut l: Logger = Logger::new(3, true, false, Some("log.txt".to_string()));
    let _ = l.log(None, "Test-Log.1");
    let _ = l.log(None, "Test-Log.2");
    let _ = l.log(None, "Test-Log.3");
}