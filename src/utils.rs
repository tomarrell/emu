use std::path;

pub fn lossy(buf: &path::PathBuf) -> String {
    buf.as_os_str().to_string_lossy().to_string()
}
