mod builder;
mod lines_reader;
mod reader;

pub use builder::IxBuilder;
pub use reader::IxReader;
use std::io::Read;
use std::path::PathBuf;

fn read_bytes<R: Read, const N: usize>(reader: &mut R) -> anyhow::Result<[u8; N]> {
    let mut bytes = [0u8; N];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

pub fn ix_path(mut path: PathBuf) -> anyhow::Result<PathBuf> {
    path.set_file_name(format!(
        "{}.ix",
        path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Missing file name"))?
            .to_string_lossy()
    ));
    Ok(path)
}
