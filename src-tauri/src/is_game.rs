use sysinfo::Process;
use std::path::Path;

/// Checks if the given process is a game.
///
/// A process is considered a game if its executable filename
/// ends with `.exe`. This function is used for filtering processes
/// when monitoring running applications.
pub fn is_game(process: &Process) -> bool {
    process.exe()
        .map(|path| is_game_path(&path))
        .unwrap_or(false)
}

fn is_game_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name_str| file_name_str.ends_with(".exe"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_path_valid_exe() {
        let path = Path::new("C:\\Games\\game.exe");
        assert!(is_game_path(&path));
    }

    #[test]
    fn test_is_game_path_invalid_exe() {
        let path = Path::new("C:\\Games\\game.txt");
        assert!(!is_game_path(&path));
    }

    #[test]
    fn test_is_game_path_no_extension() {
        let path = Path::new("C:\\Games\\game");
        assert!(!is_game_path(&path));
    }

    #[test]
    fn test_is_game_path_empty() {
        let path = Path::new("");
        assert!(!is_game_path(&path));
    }
}