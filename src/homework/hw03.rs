#[test]
fn test3() {
    let W: u32 = 30;
    let H: u32 = 10;
    let k = W as f32 / H as f32;
    for y in 0..H {
        for x in 0..W {
            
            let slash  = x == (y as f32 * k) as u32;
            let slash_rev  = W  - x == (y as f32 * k) as u32;

            let is_hor = y == 0 || y == H - 1;
            let is_vec = x == 0 || x == W - 1;

            let show = slash || slash_rev ||is_hor|| is_vec;

            let sym = if show {"*"} else {" "};

            print!("{sym}");
        }

        println!();
    }
}