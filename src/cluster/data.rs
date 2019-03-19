extern crate directories;

use std::fs;
use std::io;
use std::path::{PathBuf};

use directories::{ProjectDirs};

pub fn get_directory() -> Result<PathBuf, io::Error> {
  if let Some(proj_dirs) = ProjectDirs::from("com", "zeerorg", "k3s_in_docker") {
    return Ok(proj_dirs.data_dir().to_owned());
  }
  return Err(io::Error::new(io::ErrorKind::Other, "Couldn't get directory"));
}

pub fn create_cluster_dir(name: &str) -> Result<PathBuf, io::Error> {
  match get_directory() {
    Ok(mut path) => {
      path.push(name);
      fs::create_dir_all(path.as_path())?;
      return Ok(path)
    },
    Err(e) => {
      return Err(e);
    }
  }
}

pub fn delete_cluster_dir(name: &str) -> Result<PathBuf, io::Error> {
  match get_directory() {
    Ok(mut path) => {
      path.push(name);
      fs::remove_dir_all(path.as_path())?;
      return Ok(path)
    },
    Err(e) => {
      return Err(e);
    }
  }
}

pub fn get_cluster_path(name: &str) -> Result<PathBuf, io::Error> {
   match get_directory() {
    Ok(mut path) => {
      path.push(name);
      if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::Other, "directory doesn't exist"))
      }
      return Ok(path);
    }
    e => {
      return e;
    }
   }
}