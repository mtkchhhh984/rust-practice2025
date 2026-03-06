fn solve() -> Vec<(u32, u32, u32, u32, u32, u32, u32, u32)> {
    let mut solutions = Vec::new();

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m {
                continue;
            }
            for x in 1..=8 {
                if x == m || x == u {
                    continue;
                }
                for a in 1..=8 {
                    if a == m || a == u || a == x {
                        continue;
                    }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a {
                            continue;
                        }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s {
                                continue;
                            }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l {
                                    continue;
                                }
                                for n in 1..=8 {
                                    if n == m
                                        || n == u
                                        || n == x
                                        || n == a
                                        || n == s
                                        || n == l
                                        || n == o
                                    {
                                        continue;
                                    }

                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    if muxa * a == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

pub fn main() {
    let solutions = solve();

    for (i, (m, u, x, a, s, l, o, n)) in solutions.iter().enumerate() {
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        println!("Розв'язок {}:", i + 1);
        println!("  {}", muxa);
        println!("x    {}", a);
        println!("------");
        println!("  {}", slon);
        println!(
            "m={}, u={}, x={}, a={}, s={}, l={}, o={}, n={}",
            m, u, x, a, s, l, o, n
        );
        println!();
    }

    println!("Кількість розв'язків: {}", solutions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_exists() {
        let solutions = solve();
        assert!(!solutions.is_empty());
    }
}