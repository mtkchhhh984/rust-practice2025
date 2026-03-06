fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;

    if len == 0 {
        return s;
    }

    let mut shift = n % len;

    if shift < 0 {
        shift += len;
    }

    let shift = shift as usize;

    let left = &s[s.len() - shift..];
    let right = &s[..s.len() - shift];

    format!("{}{}", left, right)
}

fn main() {
    let s = "abcdefgh".to_string();
    println!("{}", rotate(s, 2));
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();

    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.clone(), *n), exp.to_string());
    });
}