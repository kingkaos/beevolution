use log::error;
use std::path::Path;
use std::{fs, io};

fn try_create_folder(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(_) => match fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}

fn try_create_file(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(_) => match fs::OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}

const HOME: &str = env!("HOME");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn store_path() -> String {
    format!("{}/.config/{}/config.json", HOME, PKG_NAME)
}

pub fn init_store_file() -> bool {
    // default location of the store is ~/.config/beevolution/config.json
    // The path and file will be created if they do not exist.
    let path = store_path();
    let path_to_store = Path::new(&path);
    // create folders if they don't exist
    try_create_folder(&(path_to_store.parent().unwrap()))
        .unwrap_or_else(|why| error!("Unable to create folder: {:?}", why));

    // try create store file if it doesn't exist
    try_create_file(&path_to_store)
        .unwrap_or_else(|why| error!("Unable to create file: {:?}", why));

    // returns true if file exists else false
    Path::new(&path_to_store).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_store_file() {
        // store file is created upon calling check_store_file()
        assert_eq!(init_store_file(), true);
    }
}
