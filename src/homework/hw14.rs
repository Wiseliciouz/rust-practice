pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let prev_gray = gray(n - 1);

    let mut current_gray = Vec::with_capacity(prev_gray.len() * 2);

    for code in prev_gray.iter() {
        current_gray.push(format!("0{}", code));
    }

    for code in prev_gray.iter().rev() {
        current_gray.push(format!("1{}", code));
    }

    current_gray
}


#[cfg(test)] 
mod tests {
    use super::*; 

    #[test]
    fn test_gray_code() {
        let test_data: [(u8, Vec<&str>); 5] = [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "11", "10")),
            (3, vec!("000", "001", "011", "010", "110", "111", "101", "100")),
            (4, vec!(
                "0000", "0001", "0011", "0010",
                "0110", "0111", "0101", "0100",
                "1100", "1101", "1111", "1110",
                "1010", "1011", "1001", "1000"
            )),
        ];


        test_data
            .iter()
            .for_each(|(n, out)| {
                let expected_output: Vec<String> = out.iter().map(|&s| s.to_string()).collect();
                let actual_output = gray(*n);
                println!("Testing n={}: Expected {:?}, Got {:?}", n, expected_output, actual_output);
                assert_eq!(actual_output, expected_output, "Test failed for n={}", n); 
            });
    }
}