pub fn format_seconds(seconds: f64) -> String {
    let total_min = (seconds / 60.0) as i32;
    let total_hr = (total_min / 60) as i32;
    let total_sec = (seconds % 60.0) as i32;

    if total_min < 1 {
        format!("{:.2}sec", total_sec)
    } else if total_hr < 1 {
        format!("{:02}min {:02}sec", total_min, total_sec)
    } else {
        format!("{}hr {:02}min {:02}sec", total_hr, total_min, total_sec)
    }
}
