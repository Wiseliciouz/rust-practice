#[test]
fn test3() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let slash  =  y + y * 2 == x + 1;
            let slash_rev  = y + y * 2 == W - x;
            //let slash  =  y + y * 2 == x;
            //let slash_rev  = y + y * 2 == W - x - 1;

            let is_hor = y == 0 || y == H - 1;
            let is_vec = x == 0 || x == W - 1;

            let show = slash || slash_rev ||is_hor|| is_vec;

            let sym = if show {"*"} else {" "};

            print!("{sym}");
        }

        println!();
    }
}