use std::fs;
pub fn solution_one_a() {
    let contents = fs::read_to_string("./one.txt").unwrap();

    let mut floor = 0;
    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    dbg!(floor);
}
pub fn solution_one_b() {
    let contents = fs::read_to_string("./one.txt").unwrap();

    let mut floor = 0;
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            dbg!(i + 1);
            break;
        }
    }
}
