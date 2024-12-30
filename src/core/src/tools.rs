///
/// Replace folders in paths by their content (non-recursive)
///
pub fn expand_folders(paths: &[std::path::PathBuf]) -> Vec<std::path::PathBuf> {
    let mut expanded: Vec<std::path::PathBuf> = Vec::new();
    for path in paths.iter() {
        if !path.is_dir() {
            expanded.push(path.to_path_buf());
        } else {
            match std::fs::read_dir(path) {
                // Let open display the error and process next path.
                Err(_) => expanded.push(path.to_path_buf()),
                // Add all files to expanded list
                Ok(entries) => {
                    for entry in entries {
                        let path = entry.unwrap().path();
                        // non-recursive
                        if path.is_file() {
                            expanded.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
    }
    expanded
}
