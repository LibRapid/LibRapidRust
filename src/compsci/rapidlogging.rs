//! Basic logging functionality.
use chrono::{Utc, SecondsFormat};
use std::io::prelude::*;
use std::fs::OpenOptions;

/// The structure for the RapidLogger.
pub struct Logger {
    pub buff_size:  usize,
        buff_count: usize,
        buffer:     String,
    pub console:    bool,
    pub file:       bool,
    pub file_path:  String,
}

impl Logger {
    /// Creates a new `Logger` object.
    pub fn new(buff_size: usize,
               console:   bool,
               file:      bool,
               file_path: String) -> Logger {
                Logger { buff_size,
                         buffer:     String::new(),
                         buff_count: 0,
                         console,
                         file,
                         file_path }
    }
    /// Creates a new `Logger` object with default values.
    pub fn new_default() -> Logger {
        Logger { buff_size:  10,
                 buff_count: 0,
                 buffer:     String::new(),
                 console:    true,
                 file:       false,
                 file_path:  String::new() }
    }
    /// Logs to a `Logger`.
    /// # Returns
    /// A `Result<bool, String>`. `true` if it was successful, otherwise the error message as a `String`.
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::rapidlogging::Logger;
    /// let mut l: Logger = Logger::new(3, true, true, "log.txt".to_string());
    /// let _ = l.log(None, "Test-Log.");
    /// let _ = l.log(None, "Test-Log.");
    /// let _ = l.log(None, "Test-Log.");
    /// ```
    /// As you can see, we initialise a new Logger `l` with the buffer size 3. This means that only after 3x logging, the logger writes to the file.
    /// Instead of `None`, you can also add things like `some(vec!["warning", "low urgency"])`. Then, one line of the output will look like this:
    /// `[2021-12-13T18:08:06Z][warning][low urgency] Test-Log.`
    pub fn log(&mut self, prefixes: Option<Vec<&str>>, msg: &str) -> Result<bool, String> {
        self.buff_count += 1;
        let mut out:     String = String::new();
        let mut out_cns: String = format!("[{}]", Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true));
        match prefixes {
            Some(v) => { 
                for s in v {
                    out_cns += &format!("[{}]", s);
                }
            }
            None => { }
        }
        out_cns.push(' ');
        out_cns.push_str(msg);
        out.push_str(&self.buffer);
        out.push_str(&(out_cns.clone() + "\n"));
        match self.console {
            true  => { println!("{}", out_cns); }
            false => { }
        }

        match self.file {
            true  => { 
                if self.buff_count == self.buff_size {
                    let file = OpenOptions::new()
                                                            .create(true)
                                                            .append(true)
                                                            .open(&self.file_path);
                    let mut file = match file {
                        Ok(f) => f,
                        Err(e) => { return Err(format!("Problem opening or creating file: {:?}", e)); },
                    };
                    self.buffer += &out_cns;
                    match writeln!(file, "{}", self.buffer) {
                        Ok(_) => { },
                        Err(e) => { return Err(format!("Problem writing to file: {:?}", e)); },
                    }
                } else { self.buffer = out; }
            }
            false => { return Err(format!("Problem opening file: {}", &self.file_path)); }
        }
        Ok(true)
    }
}