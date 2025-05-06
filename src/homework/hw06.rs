#[test]
fn triangles() {
    const NUMBER_OF_TREE: i32 = 7;
    let mut w_floor = 0;
    
    for c in 0..NUMBER_OF_TREE {
        let width = (NUMBER_OF_TREE * 2) - 1;
        let height = c;
        let width = width;

        let aspect_ratio = (w_floor as f32) / (height as f32);
        let center = width as f32 / 2.0 ;
    
        let place_for_ratio = |y: i32| {
            (center + y as f32 * aspect_ratio) as i32
        };
        
        let mut fill = false;
        for y in 0..height {
            for x in 0..width {
                let right = x == place_for_ratio(y);
                let left = width - x == place_for_ratio(y)+1;

                if left {
                    fill = true;
                }
                else if right 
                { 
                    fill = false;
                }

                let show = right || left;
    
                let sym = if show || fill {"*"} else {" "};
                if left && right {
                    fill = false;
                }
                print!("{sym}");
            }
            println!(" ");
        }
        w_floor += 1;
    }
}

#[test]
fn ai_upscale_my_code() {
    const NUMBER_OF_TREE: i32 = 7;
    let mut w_floor = 0;

    for level in 0..NUMBER_OF_TREE {
        let width = (NUMBER_OF_TREE * 2) - 1;
        let height = level;
        let center = width / 2;
        let aspect_ratio = if height != 0 {
            w_floor as f32 / height as f32
        } else {
            0.0
        };

        for y in 0..height {
            let offset = (y as f32 * aspect_ratio) as i32;
            let left_edge = center - offset;
            let right_edge = center + offset;

            for x in 0..width {
                let sym = if x >= left_edge && x <= right_edge {
                    "*"
                } else {
                    " "
                };
                print!("{sym}");
            }
            println!();
        }

        w_floor += 1;
    }
}
