use log::error;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn touch(path: &Path) -> io::Result<()> {
    match fs::OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn check_store_file() -> bool {
    // default location of the store is ~/.config/beevolution/config.json
    // The path and file will be created if they do not exist.
    let home = env::var("HOME").unwrap();
    let path = format!("{}/.config/beevolution", home);
    // create folders if they do not exist
    if fs::metadata(&path).is_err() {
        fs::create_dir_all(&path).unwrap_or_else(|why| {
            error!("Unable to create path {:?}", why.kind());
        });
    }
    let path_to_store = format!("{}/config.json", path);
    if fs::metadata(&path_to_store).is_err() {
        touch(&Path::new(&path_to_store)).unwrap_or_else(|why| {
            error!("Unable to create file {:?}", why.kind());
        });
    };
    // returns true if file exists else false
    Path::new(&path_to_store).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_store_file() {
        // store file is created upon calling check_store_file()
        assert_eq!(check_store_file(), true);
    }
}
