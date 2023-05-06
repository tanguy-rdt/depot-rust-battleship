pub fn get_position(pos: &str) -> (i32, i32){
    let mut chars = pos.chars();
    let letter = chars.next().unwrap();

    let mut x = 0;
    if letter.is_ascii_lowercase() {
        x = (letter as i32) - ('a' as i32);
    }
    else if letter.is_ascii_uppercase() {
        x = (letter as i32) - ('A' as i32);
    }
    
    let y = chars.as_str().parse::<i32>().unwrap();

    (x, y)
}

pub fn check_ship_position((x, y): ([i32; 2], [i32; 2]), ship: &str) -> bool {
    let x = x[1]-x[0]+1;
    let y = y[1]-y[0]+1;
    let res = match ship {
        "Aircraft carrier" => if (x == 5) || (y == 5) { true } else { false },
        "Cruiser" => if (x == 4) || (y == 4) { true } else { false },
        "Destroyer" => if (x == 3) || (y == 3) { true } else { false },
        "Submarine" => if (x == 2) || (y == 2) { true } else { false },
        _ => false,
    };

    res
}