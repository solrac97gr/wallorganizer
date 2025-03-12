use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::{env, io};

fn read_files_from_folder(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            file_paths.push(entry.path());
        }
    }
    Ok(file_paths)
}

fn check_is_img(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        let img_extensions = [
            "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "svg", "ico", "heic",
        ];
        return img_extensions.contains(&extension.to_lowercase().as_str());
    }
    false
}

fn check_if_was_operated(path: &Path) -> bool {
    if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
        let mut digit_count = 0;
        for c in filename.chars() {
            if c.is_digit(10) {
                digit_count += 1;
            } else {
                break;
            }
        }
        return digit_count >= 8;
    }
    false
}

fn get_creation_date_as_string(path: &Path) -> Option<String> {
    match fs::metadata(path) {
        Ok(metadata) => match metadata.created() {
            Ok(time) => match time.duration_since(UNIX_EPOCH) {
                Ok(duration) => Some(duration.as_secs().to_string()),
                Err(e) => {
                    eprintln!("Error calculating timestamp for {}: {}", path.display(), e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Error getting creation date for {}: {}", path.display(), e);
                None
            }
        },
        Err(e) => {
            eprintln!("Error getting metadata for {}: {}", path.display(), e);
            None
        }
    }
}

fn generate_new_filename(path: &Path, timestamp: &str) -> PathBuf {
    let parent = path.parent().unwrap_or(Path::new(""));

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    let new_filename = format!("{}_{}", timestamp, filename);

    parent.join(new_filename)
}

fn rename_file(old_path: &Path, new_path: &Path) -> Result<(), io::Error> {
    println!("Renaming: {} -> {}", old_path.display(), new_path.display());
    fs::rename(old_path, new_path)
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let wallpaper_folder = if args.len() > 1 {
        args[1].clone()
    } else {
        "./wallpapers".to_string()
    };

    println!("Processing files in: {}", wallpaper_folder);

    let mut modified_count = 0;

    match read_files_from_folder(&wallpaper_folder) {
        Ok(files) => {
            for file_path in files {
                if check_is_img(&file_path) && !check_if_was_operated(&file_path) {
                    if let Some(timestamp) = get_creation_date_as_string(&file_path) {
                        let new_path = generate_new_filename(&file_path, &timestamp);

                        match rename_file(&file_path, &new_path) {
                            Ok(_) => {
                                println!("✓ Renamed: {}", file_path.display());
                                modified_count += 1;
                            }
                            Err(e) => {
                                eprintln!("✗ Error renaming {}: {}", file_path.display(), e);
                            }
                        }
                    }
                }
            }

            println!("\nProcess completed: {} file(s) renamed", modified_count);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading wallpaper folder: {}", e);
            Err(e)
        }
    }
}
