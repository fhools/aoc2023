fn cube_count(s: &str) -> (i32, i32, i32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let sets = s.split(',');
    for set in sets {
        let mut set = set.trim().split(' ');
        let num = set.next().unwrap().parse::<i32>().unwrap();
        let color = set.next().unwrap();
        match color {
            "red" => r += num,
            "green" => g += num,
            "blue" => b += num,
            _ => panic!("Invalid color"),
        }
    }
    (r, g, b)
}
fn parse_game_part1(l: &str) -> Option<i32> {
    let mut gamenum_sets = l.split(':');
    let game_num = gamenum_sets.next().unwrap().split(' ').nth(1).unwrap();
    let game_sets = gamenum_sets.next().unwrap().split(';');
    let R = 12;
    let G = 13;
    let B = 14;
    let game_num = game_num.parse::<i32>().unwrap();
    for set in game_sets {
        let (r, g, b) = cube_count(set);
        if r > R || g > G || b > B {
            println!("Game {}: invalid", game_num);
            return None;
        }
    }
    Some(game_num)
}

fn parse_game_part2(l: &str) -> Option<i32> {
    let mut gamenum_sets = l.split(':');
    let game_num = gamenum_sets.next().unwrap().split(' ').nth(1).unwrap();
    let game_sets = gamenum_sets.next().unwrap().split(';');
    let R = 12;
    let G = 13;
    let B = 14;
    let mut r_max = 0;
    let mut g_max = 0;
    let mut b_max = 0;
    let game_num = game_num.parse::<i32>().unwrap();
    for set in game_sets {
        let (r, g, b) = cube_count(set);
        if r > r_max {
            r_max = r;
        }
        if g > g_max {
            g_max = g;
        }
        if b > b_max {
            b_max = b;
        }
    }
    if r_max == 0 || g_max == 0 || b_max == 0 {
        panic!("game with no cubes");
    }
    Some(r_max * g_max * b_max)
}
static PART1: bool = false;
fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let mut sum = 0;
    for line in input.lines() {
        if PART1 {
            if let Some(game_num) = parse_game_part1(line) {
                sum += game_num;
            }
        } else {
            if let Some(game_num) = parse_game_part2(line) {
                sum += game_num;
            }
        }
    }

    if PART1 {
        println!("Sum: {}", sum);
    } else {
        println!("Power Sum: {}", sum);
    }
}
