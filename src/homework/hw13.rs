#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut area = 0;

    let min_x = xs.iter().map(|r| r.a.x.min(r.b.x)).min().unwrap();
    let max_x = xs.iter().map(|r| r.a.x.max(r.b.x)).max().unwrap();
    let min_y = xs.iter().map(|r| r.a.y.min(r.b.y)).min().unwrap();
    let max_y = xs.iter().map(|r| r.a.y.max(r.b.y)).max().unwrap();

    for x in min_x..max_x {
        for y in min_y..max_y {
            let mut covered = false;

            for r in xs {
                let x1 = r.a.x.min(r.b.x);
                let x2 = r.a.x.max(r.b.x);
                let y1 = r.a.y.min(r.b.y);
                let y2 = r.a.y.max(r.b.y);

                if x >= x1 && x < x2 && y >= y1 && y < y2 {
                    covered = true;
                    break;
                }
            }

            if covered {
                area += 1;
            }
        }
    }

    area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

pub fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("occupied area = {}", occupied);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}