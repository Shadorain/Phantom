use std::{
    collections::VecDeque,
    fs::File,
    io::{Error, Write},
};

pub struct Log {
    queue: VecDeque<String>,
    file: Option<File>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            file: None,
        }
    }
    /// Sets up the logging file
    /// NOTE: Doesn't propogate own errors, pushes into queue
    ///
    /// * `filename`: Optional absolute/relative path to the file to log to
    pub fn with_file(&mut self, filename: Option<&str>) {
        if filename.is_none() {
            return;
        }

        let file = File::options().append(true).open(filename.unwrap());
        if let Ok(file) = file {
            self.file = Some(file);
        } else {
            self.queue
                .push_back(format!("Could not open log file: {}", filename.unwrap()));
        }
    }
    /// Push a message into the log queue
    /// Writes to the log file if setup
    /// NOTE: Does not propogate own errors, pushes into queue
    ///
    /// * `msg`: message to be logged
    pub fn push(&mut self, msg: &str) {
        self.queue.push_back(msg.to_owned());
        if self.file.is_some() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(msg.as_bytes())
                .map_err(|e| {
                    self.queue
                        .push_back(format!("Failed to write to file: {}", e))
                })
                .ok();
        }
    }
    /// Pops a message off the log queue
    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
}
