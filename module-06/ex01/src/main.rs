use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

struct Logger<W> {
    buffer: Box<[u8]>,
    buffer_len: usize,
    writer: W,
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self {
        Self {
            buffer: vec![0u8; threshold].into_boxed_slice(),
            buffer_len: 0,
            writer,
        }
    }
}

impl<W: Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let message_bytes = message.as_bytes();
        let message_len = message_bytes.len();

        if message_len > self.buffer.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Message too large"));
        }

        if self.buffer_len + message_len > self.buffer.len() {
            self.writer.write_all(&self.buffer[..self.buffer_len])?;
            self.buffer_len = 0;
        }

        self.buffer[self.buffer_len..self.buffer_len + message_len].copy_from_slice(message_bytes);
        self.buffer_len += message_len;

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.buffer_len > 0 {
            self.writer.write_all(&self.buffer[..self.buffer_len])?;
            self.buffer_len = 0;
        }
        Ok(())
    }
}

fn main() {
    let threshold = 100;
    let writer = io::stdout();
    let logger = Arc::new(Mutex::new(Logger::new(threshold, writer)));

    let mut handles = vec![];

    for i in 0..10 {
        let logger = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let message = format!("hello {} from thread {}!\n", j, i);
                let mut logger = logger.lock().unwrap();
                logger.log(&message).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut logger = logger.lock().unwrap();
    logger.flush().unwrap();
}
