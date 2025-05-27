use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

fn normalize_rectangle(rect: &Rectangle) -> Option<(i32, i32, i32, i32)> {
    let x1 = min(rect.a.x, rect.b.x);
    let y1 = min(rect.a.y, rect.b.y); 
    let x2 = max(rect.a.x, rect.b.x);
    let y2 = max(rect.a.y, rect.b.y); 

    if x1 == x2 || y1 == y2 {
        None
    } else {
        Some((x1, y1, x2, y2))
    }
}

pub fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    if xs.is_empty() {
        return 0;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let mut normalized_rects = Vec::new();
    for rect in xs {
        if let Some((x1, y1, x2, y2)) = normalize_rectangle(rect) {
            min_x = min_x.min(x1);
            max_x = max_x.max(x2);
            min_y = min_y.min(y1);
            max_y = max_y.max(y2);
            normalized_rects.push((x1, y1, x2, y2));
        }
    }

    if normalized_rects.is_empty() {
        return 0;
    }

     if max_x <= min_x || max_y <= min_y {
        return 0;
    }


    let grid_width = (max_x - min_x) as usize;
    let grid_height = (max_y - min_y) as usize;

    let mut covered = vec![vec![false; grid_width]; grid_height];

    for &(x1, y1, x2, y2) in &normalized_rects {
        for x in x1..x2 { 
            for y in y1..y2 { 

                let x_idx = (x - min_x) as usize;
                let y_idx = (y - min_y) as usize; 


                if x_idx < grid_width && y_idx < grid_height {
                    covered[y_idx][x_idx] = true;
                }
            }
        }
    }

    let mut occupied_area = 0;
    for row in covered {
        for cell_is_covered in row {
            if cell_is_covered {
                occupied_area += 1;
            }
        }
    }

    occupied_area
}



#[cfg(test)]
mod tests {
    use super::*; 

    fn test_data() -> Vec<Rectangle> {
        vec![
            Rectangle {
                a: Point { x: 2, y: 9 },
                b: Point { x: 5, y: 3 },
            },
            Rectangle {
                a: Point { x: 1, y: 8 },
                b: Point { x: 11, y: 6 },
            },
            Rectangle {
                a: Point { x: 9, y: 10 },
                b: Point { x: 13, y: 2 },
            },
        ]
    }

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }

     #[test]
     fn area_occupied_test_with_output() {
        let data = test_data();
        let occupied = area_occupied(&data);
        println!("Тестові прямокутники: {:?}", data); 
        println!("Розрахована зайнята площа: {}", occupied); 
        assert_eq!(occupied, 60);
     }
}

