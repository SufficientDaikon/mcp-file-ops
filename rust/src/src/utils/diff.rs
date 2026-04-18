pub fn unified_diff(original: &[String], modified: &[String]) -> String {
    let mut diff = String::new();
    diff.push_str(&format!("--- original\n+++ modified\n"));

    let mut i = 0;
    let mut j = 0;

    while i < original.len() || j < modified.len() {
        if i < original.len() && j < modified.len() && original[i] == modified[j] {
            i += 1;
            j += 1;
        } else if i < original.len() {
            diff.push_str(&format!("-{}\n", original[i]));
            i += 1;
        } else {
            diff.push_str(&format!("+{}\n", modified[j]));
            j += 1;
        }
    }

    diff
}
