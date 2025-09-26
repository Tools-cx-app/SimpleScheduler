use std::{
    collections::{HashMap, hash_map::Entry},
    fs::{File, set_permissions},
    io::{self, ErrorKind, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use sys_mount::{UnmountFlags, unmount};

pub struct FilesHandler {
    files: HashMap<PathBuf, File>,
}

impl FilesHandler {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn write_with_handler<P, T>(&mut self, path: P, content: T) -> Result<()>
    where
        P: AsRef<Path>,
        T: AsRef<[u8]>,
    {
        if let Err(e) = self.write(path.as_ref(), content.as_ref()) {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    set_permissions(path.as_ref(), PermissionsExt::from_mode(0o644))?;
                    self.write(path, content)?;
                    Ok(())
                }
                ErrorKind::InvalidInput => Ok(()),
                _ => Err(e.into()),
            }
        } else {
            Ok(())
        }
    }

    pub fn write<P, T>(&mut self, path: P, content: T) -> io::Result<()>
    where
        P: AsRef<Path>,
        T: AsRef<[u8]>,
    {
        match self.files.entry(path.as_ref().to_path_buf()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().write_all(content.as_ref())?;
            }
            Entry::Vacant(entry) => {
                let _ = unmount(path.as_ref(), UnmountFlags::DETACH);
                set_permissions(path.as_ref(), PermissionsExt::from_mode(0o644))?;
                let mut file = File::create(path)?;
                file.write_all(content.as_ref())?;
                entry.insert(file);
            }
        }

        Ok(())
    }
}
