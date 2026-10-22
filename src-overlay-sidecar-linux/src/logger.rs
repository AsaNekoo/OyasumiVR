use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use oyasumi_shared::get_log_path;

#[derive(Clone, Default)]
pub struct Writter {
    pub file: Arc<Mutex<Option<File>>>,
}

impl Write for Writter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(file) = self.file.lock().unwrap().as_mut() {
            file.write_all(buf)?;
        }
        io::stdout().write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(file) = self.file.lock().unwrap().as_mut() {
            file.flush()?;
        }
        io::stdout().flush()
    }
}

pub fn get_o_log_path() -> PathBuf {
    get_log_path().join("overlay.log")
}
