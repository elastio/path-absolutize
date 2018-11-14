#![cfg(windows)]

use super::Absolutize;

use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};

use path_dedot::{CWD, ParseDot, ParsePrefix};

impl Absolutize for Path {
    fn absolutize(&self) -> io::Result<PathBuf> {
        if self.is_absolute() {
            self.parse_dot()
        } else {
            let prefix = self.get_path_prefix();

            if let Some(prefix) = prefix {
                let cwd = CWD.to_str().unwrap();

                let cwd_prefix = CWD.get_path_prefix().unwrap();

                let self_str = self.to_str().unwrap();

                let path = PathBuf::from(format!(r"{}{}\{}", prefix.as_os_str().to_str().unwrap(), &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..], &self_str[prefix.as_os_str().to_str().unwrap().len()..]));

                path.parse_dot()
            } else {
                let path = Path::join(&CWD, self);

                path.parse_dot()
            }
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            let path_lowercase = path.to_str().unwrap().to_lowercase();

            let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

            if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                let path_lowercase = path.to_str().unwrap().to_lowercase();

                let virtual_root_lowercase = virtual_root.to_str().unwrap().to_lowercase();

                if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                return Ok(virtual_root);
            }
        }
    }
}