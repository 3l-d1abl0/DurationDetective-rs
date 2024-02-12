use colored::Colorize;
use std::fs;

pub fn is_valid_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    match fs::metadata(path) {
        Ok(stat) => {
            if stat.is_dir() {
                return true;
            }
            println!("{}", "ERR: Expected path to folder".red().bold());
            false
        }
        Err(err) => {
            println!("ERR: {}", err);
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    //Tests if a string is a valid folder path
    fn test_valid_folder_path() {
        //get the cwd
        let path = env::current_dir().unwrap();

        let path_str = path.to_str().unwrap();
        assert_eq!(
            is_valid_path(path_str),
            true,
            "Expected a path to a valid folder current:{}",
            path_str
        )
    }
}
