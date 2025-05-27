use std::io;
use std::vec::Vec;
use std::string::String;
use std::error::Error;
use std::iter::Sum;
use rand::Rng;
use std::io::{Read, Write, BufRead, BufWriter, Cursor}; 


pub fn count_permutation(shipments: &Vec<u32>) -> i64 {
    let n = shipments.len();
    if n == 0 {
        return 0;
    }

    let total_weight: u64 = shipments.iter().map(|&w| w as u64).sum();

    if total_weight % (n as u64) != 0 {
        return -1;
    }

    let target_weight = (total_weight / (n as u64)) as u32;

    let mut moves: u64 = 0;
    for &weight in shipments {
        if weight > target_weight {
            moves += (weight - target_weight) as u64;
        }
    }

    moves as i64
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut rng = rand::rng();
    let max_possible_item_weight: u32 = 10000;
    let base_target_weight = rng.random_range(1..=1000);

    let mut shipments: Vec<u32> = vec![base_target_weight; n];

    let num_imbalance_steps = n * 20;

    for _ in 0..num_imbalance_steps {
        let i = rng.random_range(0..n);
        let mut j = rng.random_range(0..n);
        while i == j {
             j = rng.random_range(0..n);
        }

        let max_take_from_j = shipments[j].checked_sub(1).unwrap_or(0);
        let delta_limit = max_take_from_j;

        if delta_limit > 0 {
            let delta = rng.random_range(1..=delta_limit);
            shipments[i] = shipments[i].saturating_add(delta);
            shipments[j] -= delta;
        }
    }

    debug_assert!(shipments.iter().all(|&w| w >= 1), "Generated shipment weight less than 1");

    shipments
}

fn solve<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let mut reader = io::BufReader::new(reader);
    let mut writer = io::BufWriter::new(writer);

    let mut n_str = String::new();
    reader.read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse()?;

    let mut shipments_str = String::new();
    reader.read_line(&mut shipments_str)?;
    let shipments: Vec<u32> = shipments_str
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    if shipments.len() != n {
        return Err(format!("Error: Expected {} weights, but got {}.", n, shipments.len()).into());
    }

    let min_moves = count_permutation(&shipments);

    writeln!(writer, "{}", min_moves)?;
    writer.flush()?;

    Ok(())
}

pub fn display_shipment_moves(shipments: &Vec<u32>) {
    let min_moves = count_permutation(shipments);
    println!("{}", min_moves);
}



#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_count_permutation_sample_1() {
        let shipments = vec![1, 1, 1, 1, 6];
        assert_eq!(count_permutation(&shipments), 4);
    }

    #[test]
    fn test_count_permutation_provided_example_1() {
        let shipments = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&shipments), 4);
    }

     #[test]
    fn test_count_permutation_provided_example_2() {
        let shipments = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&shipments), 7);
    }

    #[test]
    fn test_count_permutation_already_balanced() {
        let shipments = vec![5, 5, 5];
        assert_eq!(count_permutation(&shipments), 0);
    }

    #[test]
    fn test_count_permutation_single_ship() {
        let shipments = vec![10];
        assert_eq!(count_permutation(&shipments), 0);
    }

    #[test]
    fn test_count_permutation_empty_ships() {
        let shipments: Vec<u32> = vec![];
        assert_eq!(count_permutation(&shipments), 0);
    }

    #[test]
    fn test_count_permutation_impossible_case() {
        let shipments = vec![1, 2];
        assert_eq!(count_permutation(&shipments), -1);

        let shipments_impossible_2 = vec![1, 1, 2];
         assert_eq!(count_permutation(&shipments_impossible_2), -1);
    }

    #[test]
    fn test_solve_sample_1() -> Result<(), Box<dyn Error>> {
        let input = "5\n1 1 1 1 6\n";
        let mut reader = Cursor::new(input);
        let mut output_buffer = Vec::new();
        let mut writer = Cursor::new(&mut output_buffer);

        solve(&mut reader, &mut writer)?;

        let output_string = String::from_utf8(output_buffer)?;
        assert_eq!(output_string.trim(), "4");

        Ok(())
    }

    #[test]
    fn test_solve_provided_example_1() -> Result<(), Box<dyn Error>> {
         let input = "5\n8 2 2 4 4\n";
         let mut reader = Cursor::new(input);
         let mut output_buffer = Vec::new();
         let mut writer = Cursor::new(&mut output_buffer);

         solve(&mut reader, &mut writer)?;

         let output_string = String::from_utf8(output_buffer)?;
         assert_eq!(output_string.trim(), "4");

         Ok(())
    }

    #[test]
    fn test_solve_impossible_case() -> Result<(), Box<dyn Error>> {
         let input = "2\n1 2\n";
         let mut reader = Cursor::new(input);
         let mut output_buffer = Vec::new();
         let mut writer = Cursor::new(&mut output_buffer);

         solve(&mut reader, &mut writer)?;

         let output_string = String::from_utf8(output_buffer)?;
         assert_eq!(output_string.trim(), "-1");

         Ok(())
    }
    #[test]
    fn test_gen_shipments_produces_possible_cases() {
        let n = 50;
        let shipments = gen_shipments(n);
        assert_eq!(shipments.len(), n);
        let result = count_permutation(&shipments);
        assert_ne!(result, -1, "gen_shipments produced an impossible case");
        assert!(result >= 0);
        assert!(shipments.iter().all(|&w| w >= 1));
        let total_weight: u64 = shipments.iter().map(|&w| w as u64).sum();
        assert_eq!(total_weight % (n as u64), 0);
    }

    #[test]
    fn demo_display_shipment_moves() {
        println!("--- Демонстрація display_shipment_moves ---"); 
        let shipments1 = vec![8, 2, 2, 4, 4];
        println!("Вхідні ваги: {:?}", shipments1);
        print!("Вивід display_shipment_moves: ");
        display_shipment_moves(&shipments1); 
        println!("--- Кінець демонстрації ---");

        let shipments2 = vec![1, 2]; 
        println!("Вхідні ваги: {:?}", shipments2);
        print!("Вивід display_shipment_moves: ");
        display_shipment_moves(&shipments2);
        println!("--- Кінець демонстрації ---");
    }
}
