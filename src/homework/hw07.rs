pub fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let test_string = "Hello".to_string();
    println!("{} -> {}", test_string, invert_the_case(test_string.clone()));

    let cyrillic = "Привет".to_string();
    println!("{} -> {}", cyrillic, invert_the_case(cyrillic.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }
}