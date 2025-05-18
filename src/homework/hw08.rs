
fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ if n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            let mut i = 5;//11
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}




#[test]
fn test_is_prime() {
   let test_data = [
       (0, false),
       (1, false),
       (2, true),
       (3, true),
       (4, false),
       (5, true),
       (100, false),
       (10007, true),
   ];


   test_data
       .iter()
       .for_each(|(n, prime)|
           assert_eq!(is_prime(*n), *prime)
       )
}
