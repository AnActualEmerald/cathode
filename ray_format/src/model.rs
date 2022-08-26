use anyhow::Result;
use std::{collections::HashMap, path::Path};

use crate::archive::Archive;

#[derive(Default, Clone, Debug)]
pub struct Ray {
    frames: [Vec<u8>; 4],
    extras: HashMap<String, Vec<u8>>,
    meta: HashMap<String, String>,
}

impl Ray {
    pub fn get_frame(&self, index: usize) -> Option<&Vec<u8>> {
        self.frames.get(index)
    }

    pub fn set_frame(&mut self, index: usize, data: Vec<u8>) -> bool {
        if let Some(i) = self.frames.get_mut(index) {
            *i = data;
            true
        } else {
            false
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut archive = Archive::new();
        for (i, f) in self.frames.iter().enumerate() {
            if f.len() > 0 {
                archive.add_file(&format!("{}", i), f)?
            }
        }
        for (n, f) in self.extras.iter() {
            archive.add_file(&n, f)?;
        }

        let meta = serde_json::to_string(&self.meta)?;
        archive.add_file("meta.json", meta.as_bytes())?;

        Ok(())
    }
}
