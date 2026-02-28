pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f64 = bytes as f64;

    if bytes_f64 >= GB {
        format!("{:.1} GB", bytes_f64 / GB)
    } else if bytes_f64 >= MB {
        format!("{:.1} MB", bytes_f64 / MB)
    } else {
        format!("{:.1} KB", bytes_f64 / KB)
    }
}
