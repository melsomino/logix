use flate2::read::GzDecoder;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;

pub fn resolve_log_files(paths: Vec<PathBuf>) -> anyhow::Result<Vec<PathBuf>> {
    let collected = collect_paths(paths, [".tar.gz", ".tar.xz", ".log"])?;
    let mut resolved = Vec::new();
    for file_path in collected {
        resolved.extend(extract_log_files(&file_path)?);
    }
    Ok(resolved)
}

fn decompress_logs<R: Read>(
    obj: R,
    arc_path: impl AsRef<Path>,
    dest_base_path: String,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut archive = Archive::new(obj);

    let mut extracted = Vec::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        if let Some(file_name) = entry.path()?.file_name() {
            let file_name = file_name.to_string_lossy().to_string();
            if file_name.contains(".log") {
                let log_path = PathBuf::from(format!("{dest_base_path}-{file_name}"));
                println!(
                    "Extracting {file_name} from {} to {}",
                    arc_path.as_ref().to_string_lossy(),
                    log_path.to_string_lossy()
                );
                entry.unpack(&log_path)?;
                extracted.push(log_path);
            }
        }
    }
    if !extracted.is_empty() {
        std::fs::remove_file(arc_path)?;
    } else {
        println!(
            "{} does not contain log files",
            arc_path.as_ref().to_string_lossy()
        );
    }
    Ok(extracted)
}

fn extract_log_files(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let path = path.as_ref();
    let (base, arc) = if let Some(base) = strip_suffix(path, ".tar.gz") {
        (base, "gz")
    } else if let Some(base) = strip_suffix(path, ".tar.xz") {
        (base, "xz")
    } else {
        return Ok(vec![path.to_path_buf()]);
    };
    let file = std::fs::File::open(path)?;
    match arc {
        "gz" => decompress_logs(GzDecoder::new(file), path, base),
        "xz" => decompress_logs(XzDecoder::new(file), path, base),
        _ => anyhow::bail!("Unknown archive format"),
    }
}

fn strip_suffix(path: &Path, suffix: &str) -> Option<String> {
    path.to_string_lossy()
        .strip_suffix(suffix)
        .map(|x| x.to_string())
}

fn collect_files<const N: usize>(dir: &Path, substr: [&str; N]) -> anyhow::Result<Vec<PathBuf>> {
    let mut collected = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collected.extend(collect_files(&path, substr)?);
            } else if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if substr.iter().any(|suffix| {
                    filename.to_lowercase().contains(&suffix.to_lowercase())
                        && !filename.to_lowercase().ends_with(".ix")
                }) {
                    collected.push(path);
                }
            }
        }
    }

    Ok(collected)
}

pub fn collect_paths<const N: usize>(
    paths: Vec<PathBuf>,
    suffixes: [&str; N],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut collected = Vec::new();
    for path in paths {
        if path.is_file() {
            collected.push(path.clone());
        } else {
            for path in collect_files(&path, suffixes)? {
                collected.push(path);
            }
        }
    }
    Ok(collected)
}
