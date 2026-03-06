fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    let avg = total / n;

    shipments
        .iter()
        .filter(|&&x| x > avg)
        .map(|&x| (x - avg) as usize)
        .sum()
}

fn count_permutation_checked(shipments: &Vec<u32>) -> Option<usize> {
    if shipments.is_empty() {
        return Some(0);
    }

    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    Some(count_permutation(shipments))
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = 10u32;
    let mut data = vec![avg; n];

    for i in 0..n {
        if i % 2 == 0 {
            data[i] += i as u32;
        } else {
            data[i] -= (i / 2) as u32;
        }
    }

    let total: i32 = data.iter().map(|&x| x as i32).sum();
    let target = avg as i32 * n as i32;
    let diff = target - total;

    if n > 0 {
        data[0] = (data[0] as i32 + diff) as u32;
    }

    data
}

fn print_info(shipments: &Vec<u32>) {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    println!("shipments = {:?}", shipments);
    println!("total = {}", total);

    if total % n != 0 {
        println!("equal distribution is impossible");
        println!("answer = -1");
        return;
    }

    let avg = total / n;
    println!("average = {}", avg);
    println!("answer = {}", count_permutation(shipments));
}

fn main() {
    let a = vec![8, 2, 2, 4, 4];
    let b = vec![9, 3, 7, 2, 9];
    let c = vec![1, 2, 4];
    let d = gen_shipments(6);

    print_info(&a);
    println!();

    print_info(&b);
    println!();

    print_info(&c);
    println!();

    print_info(&d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(count_permutation_checked(&vec![8, 2, 2, 4, 4]), Some(4));
        assert_eq!(count_permutation_checked(&vec![9, 3, 7, 2, 9]), Some(7));
    }

    #[test]
    fn test_impossible() {
        assert_eq!(count_permutation_checked(&vec![1, 2, 4]), None);
    }

    #[test]
    fn test_generated() {
        let v = gen_shipments(10);
        assert!(count_permutation_checked(&v).is_some());
    }
}