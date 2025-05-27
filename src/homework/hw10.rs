fn is_palindrome(x: u32) -> bool {
    let mut backward = String::new();
        let s = x.to_string();
        let s = s.trim().chars();
        for ch in s.rev() {
            backward.push(ch);
        }
        let backward_int = backward.trim().parse::<i32>();
        match backward_int {
            Ok(val) => {
                if val == x.try_into().unwrap() {
                    return true;
                }
                else {
                    return false;
                }
            }
            Err(_err) => {
                return false;
            }
        }
}


#[test]
fn test() {
   let data =
       [
           (123, false),
           (121, true),
           (1221, true),
       ];


   data
       .iter()
       .for_each(|(n, exp)| {
           assert_eq!(is_palindrome(*n), *exp);
       });
}
