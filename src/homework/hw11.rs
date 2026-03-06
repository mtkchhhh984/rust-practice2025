use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = data[0] + data[1];
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_index + 1, min_sum)
}

fn print_vector_info(data: &[i32]) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>4}.", i);
    }
    println!();

    println!("data: {:?}", data);

    let (i, j, sum) = min_adjacent_sum(data);

    print!("indexes:");
    for k in 0..data.len() {
        if k == i {
            print!("   \\");
        } else if k == j {
            print!("   /");
        } else {
            print!("    ");
        }
    }
    println!();

    print!("        ");
    for k in 0..data.len() {
        if k == i || k == j {
            print!(" __ ");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i], data[j], sum, i, j
    );
}

fn main() {
    for _ in 0..4 {
        let data = gen_random_vector(20);
        print_vector_info(&data);
        println!();
    }
}