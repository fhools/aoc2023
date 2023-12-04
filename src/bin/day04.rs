static PART1: bool = false;

#[derive(Debug, Clone)]
struct Card {
    cardnum: i32,
    winnings: Vec<i32>,
    picks: Vec<i32>,
}

fn extract_card(l: &str) -> Card {
    let cards = l.split(":").collect::<Vec<_>>();
    let cardnum = cards[0].split_whitespace().collect::<Vec<_>>()[1]
        .parse::<i32>()
        .unwrap();
    let nums = cards[1].split("|").collect::<Vec<_>>();

    let winnings = nums[0].split_whitespace().collect::<Vec<_>>();
    let picks = nums[1].split_whitespace().collect::<Vec<_>>();

    // for w in winnings.iter() {
    //     println!("winning: {}", w);
    // }
    let winnings_nums = winnings
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let picks_nums = picks
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    Card {
        cardnum: cardnum,
        winnings: winnings_nums,
        picks: picks_nums,
    }
}
fn points_per_card(index: usize, cards: &Vec<Card>) -> i32 {
    let mut winning_picks = Vec::new();
    for p in cards[index].picks.iter() {
        for w in cards[index].winnings.iter() {
            if *p == *w {
                winning_picks.push(*p);
            }
        }
    }

    if winning_picks.len() == 0 {
        1
    } else {
        // the original card count as one point
        let mut card_points = 1;
        for i in 0..winning_picks.len() {
            card_points += points_per_card(index + (i as usize) + 1, cards)
        }
        card_points
    }
}

fn points(l: &str) -> i32 {
    let cards = l.split(":").collect::<Vec<_>>();
    let cardnum = cards[0].split_whitespace().collect::<Vec<_>>()[1]
        .parse::<i32>()
        .unwrap();
    let nums = cards[1].split("|").collect::<Vec<_>>();

    let winnings = nums[0].split_whitespace().collect::<Vec<_>>();
    let picks = nums[1].split_whitespace().collect::<Vec<_>>();

    // for w in winnings.iter() {
    //     println!("winning: {}", w);
    // }
    let winnings_nums = winnings
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let picks_nums = picks
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    // for w in winnings_nums.iter() {
    //     println!("winning: {}", w);
    // }

    //println!("cardnum: {}", cardnum);
    let mut winning_picks = Vec::new();
    for p in picks_nums.iter() {
        for w in winnings_nums.iter() {
            if *p == *w {
                winning_picks.push(*p);
            }
        }
    }
    //println!("winning picks of card {}: {:?}", cardnum, winning_picks);
    if winning_picks.len() == 0 {
        0
    } else {
        let points = 2i32.pow((winning_picks.len() - 1) as u32);
        points
    }
}
fn main() {
    let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let input = include_str!("../../inputs/day04.txt");
    let lines = input.lines().collect::<Vec<_>>();
    if PART1 {
        let mut total_points = 0;
        for l in lines {
            total_points += points(l);
        }
        println!("total points: {}", total_points);
    } else {
        let cards = lines.iter().map(|l| extract_card(l)).collect::<Vec<_>>();
        let mut total_card_points = 0;
        for i in 0..cards.len() {
            total_card_points += points_per_card(i, &cards);
        }
        println!("total card points: {}", total_card_points);
    }
}
