#[test]
fn rhombus() {
    const HEIGHT: i32 = 10;
    const WIDTH: i32 = 27;

    let aspect_ratio = (WIDTH as f32 / 2.0) / (HEIGHT as f32 / 2.0);
    let center = WIDTH as f32 / 2.0 - 1.0;

    let place_for_ratio = |y: i32, reverse: bool| {
        let ratio = if reverse { -aspect_ratio } else { aspect_ratio };
        (center + y as f32 * ratio) as i32
    };

    let mut fill = false;

    for y in 1..HEIGHT {
        for x in 1..WIDTH {
            let up_right = x == place_for_ratio(y, false);
            let up_left = WIDTH - x == place_for_ratio(y, false);
            let down_left = x == -place_for_ratio(y, true);
            let down_right = WIDTH - x == -place_for_ratio(y, true);

            if up_left || down_left {
                fill = true;
            } else if up_right || down_right {
                fill = false;
            }

            let border = up_left || up_right || down_left || down_right;
            let sym = if fill || border { "*" } else { " " };

            if (up_left && up_right) || (down_left && down_right) {
                fill = false;
            }
            print!("{sym}");
        }
        println!();
    }
}

