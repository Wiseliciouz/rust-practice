use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_elephant_puzzle() -> Vec<HashMap<char, u8>> {

    let digits: Vec<u8> = (1..=8).collect();


    let letters = vec!['М', 'У', 'Х', 'А', 'С', 'Л', 'О', 'Н'];

    let mut solutions: Vec<HashMap<char, u8>> = Vec::new();

    for permutation in digits.into_iter().permutations(8) {

        let mut current_assignment: HashMap<char, u8> = HashMap::new();
        for (i, &digit) in permutation.iter().enumerate() {
            current_assignment.insert(letters[i], digit);
        }

        let m = *current_assignment.get(&'М').unwrap();
        let u = *current_assignment.get(&'У').unwrap();
        let x = *current_assignment.get(&'Х').unwrap();
        let a = *current_assignment.get(&'А').unwrap();
        let s = *current_assignment.get(&'С').unwrap();
        let l = *current_assignment.get(&'Л').unwrap();
        let o = *current_assignment.get(&'О').unwrap();
        let n = *current_assignment.get(&'Н').unwrap();


        if a == 1 || a == 5 {
            continue;
        }

        let num_muha = (m as u32) * 1000 + (u as u32) * 100 + (x as u32) * 10 + (a as u32);


        let product = num_muha * (a as u32);

        if product >= 1000 && product <= 9999 {

            let derived_s = (product / 1000) as u8;
            let derived_l = ((product / 100) % 10) as u8;
            let derived_o = ((product / 10) % 10) as u8;
            let derived_n = (product % 10) as u8;

            if s == derived_s && l == derived_l && o == derived_o && n == derived_n {
                solutions.push(current_assignment);
            }
        }
    }

    solutions
}


pub fn print_solution(solution: &HashMap<char, u8>) {
    let m = solution[&'М'];
    let u = solution[&'У'];
    let x = solution[&'Х'];
    let a = solution[&'А'];
    let s = solution[&'С'];
    let l = solution[&'Л'];
    let o = solution[&'О'];
    let n = solution[&'Н'];



    let muha_str = format!("{}{}{}{}", m, u, x, a);
    let slon_str = format!("{}{}{}{}", s, l, o, n);
    let a_str = a.to_string();

    let num_width = 4;

    let multiplier_line_width = 1 + 1 + a_str.len();


    println!("  {}", muha_str); 

    println!("x{: >width$}", a_str, width = num_width); 
    println!("  ------"); 
    println!("  {}", slon_str); 
    println!(); 
}
#[test]
fn test1() {
    let solutions = solve_elephant_puzzle();

    println!("Знайдено {} рішень:", solutions.len());
    println!();

    for solution in solutions {
        print_solution(&solution);
    }
}
