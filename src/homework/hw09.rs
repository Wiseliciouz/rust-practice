
fn rotate(s: String, n: isize) -> String {
    let len = s.chars().count();
    if len == 0 {
        return s;
    }
    let shift = ((n % len as isize + len as isize) % len as isize) as usize;
    let chars: Vec<char> = s.chars().collect();
    let mut rotated = String::with_capacity(len);
    rotated.extend(chars[len - shift..].iter());
    rotated.extend(chars[..len - shift].iter());
    rotated
}



#[test]
fn test() {
   let s = "abcdefgh";
   let shifts = [
       (0,  "abcdefgh"),
       (8,  "abcdefgh"),
       (-8, "abcdefgh"),
       (1,  "habcdefg"),
       (2,  "ghabcdef"),
       (10, "ghabcdef"),
       (-1, "bcdefgha"),
       (-2, "cdefghab"),
       (-10,"cdefghab"),
   ];


   shifts
       .iter()
       .for_each(|(n, exp)|
           assert_eq!(
               rotate(s.to_string(), *n), 
               exp.to_string()
           )
       );
}
