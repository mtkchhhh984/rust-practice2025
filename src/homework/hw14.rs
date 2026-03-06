fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    result.extend(prev.iter().map(|s| format!("0{}", s)));
    result.extend(prev.iter().rev().map(|s| format!("1{}", s)));

    result
}