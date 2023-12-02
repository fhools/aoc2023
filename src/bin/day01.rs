static PART1: bool = false;

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    if PART1 {
        let mut sum = 0;
        for line in input.lines() {
            let first = line.chars().find(|&c| c.is_digit(10)).unwrap();
            let last = line.chars().rev().find(|&c| c.is_digit(10)).unwrap();
            let val = format!("{}{}", first, last).parse::<i32>().unwrap();
            sum += val;
        }
        println!("{}", sum);
    } else {
        let mut sum = 0;
        let nums = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for line in input.lines() {
            // println!("line: {}", line);
            let mut first_num = None;
            let mut second_num = None;
            let mut wordpos = vec![];

            for i in 0..line.len() {
                if line.chars().nth(i).unwrap().is_digit(10) {
                    wordpos.push(i);
                } else {
                    for j in 0..nums.len() {
                        if line[i..].starts_with(nums[j]) {
                            wordpos.push(i);
                        }
                    }
                }
            }
            wordpos.sort();
            // get first number, if its a word convert to digit, otherwise use digit char

            if line.chars().nth(wordpos[0]).unwrap().is_digit(10) {
                first_num = Some(line.chars().nth(wordpos[0]).unwrap().to_digit(10).unwrap());
            } else {
                for j in 0..nums.len() {
                    if line[wordpos[0]..].starts_with(nums[j]) {
                        first_num = Some(j as u32);
                    }
                }
            }

            // now get the last number, if its a word convert to digit, otherwise use digit char

            if line
                .chars()
                .nth(*wordpos.last().unwrap())
                .unwrap()
                .is_digit(10)
            {
                second_num = Some(
                    line.chars()
                        .nth(*wordpos.last().unwrap())
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                );
            } else {
                for j in 0..nums.len() {
                    if line[*wordpos.last().unwrap()..].starts_with(nums[j]) {
                        second_num = Some(j as u32);
                    }
                }
            }

            // now we have the first and last numbers, add them to the sum
            // the formula is first_num*10 + second_num
            sum += (first_num.unwrap() * 10) + second_num.unwrap();
        }

        println!("{}", sum);
    }
}
