use chrono::Utc;

pub fn log(level: &str, message: &str) {
    let timestamp = Utc::now();
    println!("[{}] [{}] {}", timestamp.to_rfc3339(), level, message);
}
