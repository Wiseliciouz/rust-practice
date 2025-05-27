use rand::Rng;


fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    (min_index, min_sum)
}


fn print_vector_with_min_pair(data: &[i32]) {
    let (min_index, min_sum) = min_adjacent_sum(data);

    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>5}.", i);
    }
    println!();

    print!("data:   ");
    for val in data {
        print!("{:>5},", val);
    }
    println!();

    let prefix_len = "indexes:".len();
    let arrow_pos = prefix_len + min_index * 6; 
    println!(
        "{:>width$}\\__ __/",
        "",
        width = arrow_pos
    );


    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}

#[test]
fn test() {
    let vec = gen_random_vector(20);
    print_vector_with_min_pair(&vec);
}
