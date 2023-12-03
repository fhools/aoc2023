use std::collections::HashSet;

static PART1: bool = false;

fn parts_sum(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();
    let mut set = HashSet::new();

    for y in 0..height {
        let mut x = 0;
        while x < width {
            if grid[y][x].is_digit(10) {
                let start = x;
                let mut pass = false;
                //println!("{} {}", x, y);
                while x < width && grid[y][x].is_digit(10) {
                    pass |= check_symbol(grid, x, y);
                    x += 1;
                }
                let end = x;
                if pass {
                    let val = grid[y][start..end]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    //println!("line {} val: {}, start: {}, end: {}", y, val, start, end);
                    sum += val;
                }
            } else {
                if grid[y][x] != '.' && !grid[y][x].is_digit(10) {
                    set.insert(grid[y][x]);
                    println!("char: {}", grid[y][x]);
                }
                x += 1;
            }
        }
    }
    println!("set: {:?}", set);
    sum
}

fn gear_sum(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        let mut x = 0;
        while x < width {
            if grid[y][x] == '*' {
                sum += gear_check(grid, x, y)
            }
            x += 1;
        }
    }
    sum
}

fn gear_check(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut parts = HashSet::new();
    for (dy, dx) in vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let xcheck = x as i32 + dx;
        let ycheck = y as i32 + dy;
        let width = grid[0].len();
        if xcheck >= 0 && xcheck < grid[0].len() as i32 && ycheck >= 0 && ycheck < grid.len() as i32
        {
            let mut x = xcheck as usize;
            let y = ycheck as usize;
            println!("checking {} {}", x, y);
            if grid[y][x].is_digit(10) {
                // scan for start of digit
                while grid[y][x].is_digit(10) {
                    if x == 0 {
                        break;
                    }
                    x -= 1;
                }
                let start;
                if grid[y][x].is_digit(10) {
                    start = x;
                } else {
                    start = x + 1;
                }
                //let start = if x > 0 { x + 1 } else { x };
                let mut x = start;
                while x < width && grid[y][x].is_digit(10) {
                    x += 1;
                    //println!("grabbing num {} {}", x, y);
                }
                let end = x;
                println!("{} {} {} {}", x, y, start, end);
                let val = grid[y][start..end]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                parts.insert(val);
            }
        }
    }
    if parts.len() == 2 {
        let mut iter = parts.iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        return a * b;
    } else {
        return 0;
    }
}
fn check_symbol(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for (dy, dx) in vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let xcheck = x as i32 + dx;
        let ycheck = y as i32 + dy;
        if xcheck >= 0 && xcheck < grid[0].len() as i32 && ycheck >= 0 && ycheck < grid.len() as i32
        {
            if "=*+$#@%/&-".contains(grid[ycheck as usize][xcheck as usize]) {
                return true;
            }
        }
    }
    false
}
fn main() {
    let input = include_str!("../../inputs/day03.txt");
    // #[allow(unused_variables)]
    // let input = vec![
    //     "467..114..".to_string().into_bytes(),
    //     "...*......".to_string().into_bytes(),
    //     "..35..633.".to_string().into_bytes(),
    //     "......#...".to_string().into_bytes(),
    //     "617*......".to_string().into_bytes(),
    //     ".....+.58.".to_string().into_bytes(),
    //     "..592.....".to_string().into_bytes(),
    //     "......755.".to_string().into_bytes(),
    //     "...$.*....".to_string().into_bytes(),
    //     ".664.598..".to_string().into_bytes(),
    // ];

    if PART1 {
        let mut grid = input
            .lines()
            .map(|l| l.chars().map(|c| c as char).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let sum = parts_sum(&grid);
        println!("part1: {}", sum);
    } else {
        let mut grid = input
            .lines()
            .map(|l| l.chars().map(|c| c as char).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let sum = gear_sum(&grid);
        println!("part2: {}", sum);
    }

    println!("part1: {}", 0);
}
