/// https://practice.course.rs/basic-types/intro.html
#[test]
fn test41() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x.try_into().unwrap();
    
    let z = 10; // Type of z ? 

    println!("Success!");
}