use sha2::{Sha256, Digest};

pub fn content_hash(lines: &[String]) -> String {
    let mut hasher = Sha256::new();
    for line in lines {
        hasher.update(line.as_bytes());
        hasher.update(&[b'\n']);
    }
    format!("{:x}", hasher.finalize())[..16].to_string()
}
