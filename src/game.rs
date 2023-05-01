pub struct User {
    pub solution: [[&'static str; 10]; 10],
    pub state: [[&'static str; 10]; 10],
}

fn show_solution(user: User){

}

pub fn get_position(pos: &str) -> (i32, i32){
    let mut chars = pos.chars();
    let letter = chars.next().unwrap();

    let x = (letter as i32) - ('A' as i32);
    let y = chars.as_str().parse::<i32>().unwrap();

    (x, y)
}

pub fn check_ship_position((x, y): ([i32; 2], [i32; 2]), ship: &str) -> bool {
    let res = match ship {
        "Aircraft carrier" => if (x[1]-x[0] == 5) || (y[1]-y[0] == 5) { true } else { false },
        "Cruiser" => if (x[1]-x[0] == 4) || (y[1]-y[0] == 4) { true } else { false },
        "Destroyer" => if (x[1]-x[0] == 3) || (y[1]-y[0] == 3) { true } else { false },
        "Submarine" => if (x[1]-x[0] == 2) || (y[1]-y[0] == 2) { true } else { false },
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