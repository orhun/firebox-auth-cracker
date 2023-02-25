use std::fs::File;
use std::io::{self, BufRead};

/// Buffered reader implementation.
pub struct BufReader {
    reader: io::BufReader<File>,
}

impl BufReader {
    /// Opens a file for read.
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        Ok(Self { reader })
    }

    /// Reads the next line.
    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> Option<io::Result<&'buf mut String>> {
        buffer.clear();
        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }
}
