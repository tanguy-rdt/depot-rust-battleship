pub struct User {
    pub solution: [[&'static str; 10]; 10],
    pub state: [[&'static str; 10]; 10],
}

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

pub fn fill_user_solution((x, y): ([i32; 2], [i32; 2]), user: &mut User){
    if x[0] == x[1] {
        for i in y[0]..y[1]+1{
            user.solution[x[0] as usize][i as usize] = "x";
        }
    }
    else {
        for i in x[0]..x[1]+1{
            user.solution[i as usize][y[0] as usize] = "x";
        }
    }
}

pub fn fill_user_state((x, y): (i32, i32), user: &mut User){
    if user.solution[x as usize][y as usize] == "x" {
        user.state[x as usize][y as usize] = "x";
    }
    else {
        user.state[x as usize][y as usize] = " ";
    }
}

pub fn end(user1: &User, user2: &User) -> bool{
    for x in 0..9 {
        for y in 0..9 {
            if (user1.solution[x][y] == "x") && (user1.state[x][y] != "x"){
                return false
            }

            if (user2.solution[x][y] == "x") && (user2.state[x][y] != "x"){
                return false
            }
        }
    }

    true
}