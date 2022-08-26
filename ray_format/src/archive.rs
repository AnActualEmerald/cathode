use anyhow::Context;
use anyhow::Result;
use std::io;
use std::io::Cursor;
use std::io::Write;

use zip::{write::FileOptions, ZipArchive, ZipWriter};

pub(crate) struct Archive {
    buffer: Vec<u8>,
}

impl Archive {
    pub(crate) fn new() -> Self {
        let mut buffer = vec![];
        ZipWriter::new(Cursor::new(&mut buffer));
        Self { buffer }
    }

    pub(crate) fn buffer(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    pub(crate) fn add_file(&mut self, file_name: &str, data: &[u8]) -> Result<()> {
        let mut zip = ZipWriter::new_append(Cursor::new(&mut self.buffer))?;
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
        zip.start_file(file_name, options)?;
        zip.write(data)?;
        zip.finish()?;

        Ok(())
    }

    pub(crate) fn get_file(&self, file_name: &str) -> Result<Vec<u8>> {
        let mut buffer = vec![];
        let mut zip = ZipArchive::new(Cursor::new(&self.buffer))?;

        let mut f = zip
            .by_name(file_name)
            .context("Couldn't find a file by that name")?;
        io::copy(&mut f, &mut Cursor::new(&mut buffer))?;

        Ok(buffer)
    }
}
