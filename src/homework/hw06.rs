const TRIANGLES: usize = 6; 

pub fn main() {
    let max_stars = 2 * TRIANGLES - 1;

    let tree = (1..=TRIANGLES)
        .flat_map(|t| {
            (1..=t).map(move |row| {
                let stars = 2 * row - 1;
                let pad = (max_stars - stars) / 2;
                format!("{}{}\n", " ".repeat(pad), "*".repeat(stars))
            })
        })
        .collect::<String>();

    print!("{}", tree);
    println!();
}