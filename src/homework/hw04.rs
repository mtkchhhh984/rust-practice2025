const WIDTH: usize = 11; 
const HEIGHT: usize = 11; 

pub fn main() {
    let mut out = String::new();

    let cx: isize = (WIDTH as isize - 1) / 2;
    let cy: isize = (HEIGHT as isize - 1) / 2;

    for y in 0..HEIGHT {
        let dy = (y as isize - cy).abs();
        let half = cx - dy; 

        for x in 0..WIDTH {
            let dx = (x as isize - cx).abs();

            if dx <= half {
                out.push('*');
            } else {
                out.push(' ');
            }

            out.push(' ');
        }
        out.push('\n');
    }

    print!("{}", out);
    println!();
}