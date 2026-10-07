use std::path::Path;

use libloading::Library;

pub struct ConfigLoader {
    library: Library,
}

impl ConfigLoader {
    pub fn load(
        library_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let library = unsafe {
            Library::new(library_path)?
        };

        Ok(Self {
            library,
        })
    }

    pub fn library(&self) -> &Library {
        &self.library
    }
}