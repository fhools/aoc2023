use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
static PART1: bool = false;
fn cmptest() {
    let a = "KKJTK";
    let b = "KK9KK";
    let mut suittopoints = HashMap::new();
    suittopoints.insert('A', 13);
    suittopoints.insert('K', 12);
    suittopoints.insert('Q', 11);
    suittopoints.insert('T', 10);
    suittopoints.insert('9', 9);
    suittopoints.insert('8', 8);
    suittopoints.insert('7', 7);
    suittopoints.insert('6', 6);
    suittopoints.insert('5', 5);
    suittopoints.insert('4', 4);
    suittopoints.insert('3', 3);
    suittopoints.insert('2', 2);
    suittopoints.insert('J', 1);

    for (i, ahand) in a.chars().enumerate() {
        let bhand = b.chars().nth(i).unwrap();
        if ahand != bhand {
            let aval = suittopoints.get(&ahand).unwrap();
            let bval = suittopoints.get(&bhand).unwrap();
            if aval < bval {
                println!(
                    "{} {} LESSTHAN {} {} i: {} aval:{} bval: {}",
                    a, ahand, b, bhand, i, aval, bval
                );
                return;
            } else if aval > bval {
                println!(
                    "{} LESSTHAN {} i: {} aval:{} bval: {}",
                    ahand, bhand, i, aval, bval
                );
                return;
            }
        }
    }
}
fn f(hand: &(String, i32)) -> Option<i32> {
    let mut rankmap = Vec::new();
    let mut suittopoints = HashMap::new();
    suittopoints.insert('A', 13);
    suittopoints.insert('K', 12);
    suittopoints.insert('Q', 11);
    suittopoints.insert('J', 10);
    suittopoints.insert('T', 9);
    suittopoints.insert('9', 8);
    suittopoints.insert('8', 7);
    suittopoints.insert('7', 6);
    suittopoints.insert('6', 5);
    suittopoints.insert('5', 4);
    suittopoints.insert('4', 3);
    suittopoints.insert('3', 2);
    suittopoints.insert('2', 1);

    //A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    rankmap.push(('A', 13));
    rankmap.push(('K', 12));
    rankmap.push(('Q', 11));
    rankmap.push(('J', 10));
    rankmap.push(('T', 9));
    rankmap.push(('9', 8));
    rankmap.push(('8', 7));
    rankmap.push(('7', 6));
    rankmap.push(('6', 5));
    rankmap.push(('5', 4));
    rankmap.push(('4', 3));
    rankmap.push(('3', 2));
    rankmap.push(('2', 1));

    for rm in rankmap {
        let s = rm.0;
        let srank = rm.1;
        let mut count = hand.0.chars().filter(|x| *x == s).count();
        if count == 5 {
            return Some(7);
        } else if count == 4 {
            return Some(6);
        } else if count == 3 {
            let otherchars = hand.0.chars().filter(|x| *x != s).collect::<String>();
            if otherchars.chars().nth(0).unwrap() == otherchars.chars().nth(1).unwrap() {
                return Some(5);
            } else {
                return Some(4);
            }
        } else if count == 2 {
            let otherchars = hand.0.chars().filter(|x| *x != s).collect::<String>();
            for oc in otherchars.chars() {
                if hand.0.chars().filter(|x| *x == oc).count() == 3 {
                    return Some(5);
                }
            }
            for oc in otherchars.chars() {
                if hand.0.chars().filter(|x| *x == oc).count() == 2 {
                    return Some(3);
                }
            }

            return Some(2);
        } else {
            let all_unique = hand.0.chars().collect::<HashSet<_>>().len() == hand.0.len();
            if all_unique {
                return Some(1);
            }
        }
    }
    None
}

// final attempt. works!
fn f3(mut hand: Vec<char>) -> Option<i32> {
    let unique = hand.iter().map(|x| *x).collect::<HashSet<_>>();
    if hand.contains(&'J') {
        let countj = hand.iter().filter(|x| **x == 'J').count();
        if countj == 5 {
            return Some(7);
        }
        let mut unique = unique.iter().map(|x| *x).collect::<HashSet<_>>();
        unique.remove(&'J');

        let mut score = 0;
        for s in unique {
            let newhand = hand
                .iter()
                .map(|x| if *x == 'J' { s } else { *x })
                .collect::<Vec<_>>();
            println!(
                "hand: {:?} newhand: {:?}",
                hand.iter().collect::<String>(),
                newhand
            );
            let newscore = scoreit(newhand);
            println!(
                "hand: {:?} score: {}",
                hand.iter().collect::<String>(),
                newscore
            );
            if newscore >= score {
                score = newscore;
            }
        }
        Some(score)
    } else {
        Some(scoreit(hand))
    }
}

fn scoreit(mut hand: Vec<char>) -> i32 {
    // convert to a new vector of a count of each char
    let mut countmap = HashMap::new();
    for c in &hand {
        let count = countmap.entry(c).or_insert(0);
        *count += 1;
    }
    let mut countmap = countmap.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
    countmap.sort_by(|a, b| a.1.cmp(&b.1));
    countmap.reverse();
    println!("hand: {:?} countmap: {:?}", hand, countmap);
    match countmap.iter().map(|x| x.1).collect::<Vec<_>>().as_slice() {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => panic!("should not get here"),
    }
}

// horrible buggy code did not work
fn f2(hand: &(String, i32)) -> Option<i32> {
    let mut rankmap = Vec::new();
    // let mut suittopoints = HashMap::new();
    // suittopoints.insert('A', 13);
    // suittopoints.insert('K', 12);
    // suittopoints.insert('Q', 11);
    // suittopoints.insert('J', 10);
    // suittopoints.insert('T', 9);
    // suittopoints.insert('9', 8);
    // suittopoints.insert('8', 7);
    // suittopoints.insert('7', 6);
    // suittopoints.insert('6', 5);
    // suittopoints.insert('5', 4);
    // suittopoints.insert('4', 3);
    // suittopoints.insert('3', 2);
    // suittopoints.insert('2', 1);

    //A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    rankmap.push(('A', 13));
    rankmap.push(('K', 12));
    rankmap.push(('Q', 11));
    rankmap.push(('J', 10));
    rankmap.push(('T', 9));
    rankmap.push(('9', 8));
    rankmap.push(('8', 7));
    rankmap.push(('7', 6));
    rankmap.push(('6', 5));
    rankmap.push(('5', 4));
    rankmap.push(('4', 3));
    rankmap.push(('3', 2));
    rankmap.push(('2', 1));

    let mut score = 0;
    for rm in rankmap {
        let s = rm.0;
        let mut count = hand.0.chars().filter(|x| *x == s).count();
        if count == 5 {
            // full house
            if 7 >= score {
                score = 7;
            }
        } else if count == 4 {
            let isotherj = hand
                .0
                .chars()
                .filter(|x| *x != s)
                .collect::<String>()
                .chars()
                .nth(0)
                .unwrap()
                == 'J';
            // full house
            if isotherj {
                if 7 >= score {
                    score = 7;
                }
            } else {
                // four of a kind
                if 6 >= score {
                    score = 6;
                }
            }
        } else if count == 3 {
            // three of a kind and one pairs
            // or three of a kind

            let otherchars = hand.0.chars().filter(|x| *x != s).collect::<String>();
            let countj = otherchars.chars().filter(|x| *x == 'J').count();
            let othercharsnotj = otherchars.chars().filter(|x| *x != 'J').collect::<String>();
            if s == 'J' {
                for oc in othercharsnotj.chars() {
                    if othercharsnotj.chars().filter(|x| *x == oc).count() == 2 {
                        // full house
                        if 7 >= score {
                            score = 7;
                        }
                    } else if othercharsnotj.chars().filter(|x| *x == oc).count() == 1 {
                        // four of a kind
                        if 6 >= score {
                            score = 6;
                        }
                    }
                }
            } else {
                if countj == 2 {
                    // full house
                    if 7 >= score {
                        score = 7;
                    }
                } else if countj == 1 {
                    // three of a kind and one pair
                    // or three of a kind
                    for oc in othercharsnotj.chars() {
                        if othercharsnotj.chars().filter(|x| *x == oc).count() == 1 {
                            // three of a kind and one pair
                            if 5 >= score {
                                score = 5;
                            }
                        } else {
                            // three of a kind
                            if 4 >= score {
                                score = 4;
                            }
                        }
                    }
                } else {
                    // three of a kind
                    if 4 >= score {
                        score = 4;
                    }
                }
            }
        } else if count == 2 {
            let otherchars = hand.0.chars().filter(|x| *x != s).collect::<String>();
            let countj = otherchars.chars().filter(|x| *x == 'J').count();
            let othercharsnotj = otherchars.chars().filter(|x| *x != 'J').collect::<String>();
            if s != 'J' {
                if countj == 3 {
                    // full house
                    if 7 >= score {
                        score = 7;
                    }
                } else if countj == 2 {
                    // four of a kind
                    if 6 >= score {
                        score = 6;
                    }
                } else if countj == 1 {
                    // three of a kind and one pair
                    // or three of a kind
                    for oc in othercharsnotj.chars() {
                        if othercharsnotj.chars().filter(|x| *x == oc).count() == 1 {
                            // three of a kind and one pair
                            if 5 >= score {
                                score = 5;
                            }
                        } else {
                            // three of a kind
                            if 4 >= score {
                                score = 4;
                            }
                        }
                    }
                } else {
                    for oc in othercharsnotj.chars() {
                        if othercharsnotj.chars().filter(|x| *x == oc).count() == 2 {
                            // two pairs
                            if 3 >= score {
                                score = 3;
                            }
                        } else if othercharsnotj.chars().filter(|x| *x == oc).count() == 1 {
                            // pair
                            if 2 >= score {
                                score = 2;
                            }
                        }
                    }
                }
                // one pair
                if 2 > score {
                    score = 2;
                }
            } else {
                // S = 2 and is J
                // 3 more char to check
                // other char is J
                for oc in othercharsnotj.chars() {
                    if othercharsnotj.chars().filter(|x| *x == oc).count() == 3 {
                        // full house
                        if 7 >= score {
                            score = 7;
                        }
                    } else if othercharsnotj.chars().filter(|x| *x == oc).count() == 2 {
                        // four of a kind
                        if 6 >= score {
                            score = 6;
                        }
                    } else if othercharsnotj.chars().filter(|x| *x == oc).count() == 1 {
                        // pair
                        if 2 >= score {
                            score = 2;
                        }
                    }
                }
            }
        } else if count == 1 {
            // count is 1
            let all_unique = hand.0.chars().collect::<HashSet<_>>().len() == hand.0.len();
            let otherchars = hand.0.chars().filter(|x| *x != s).collect::<String>();
            let countj = otherchars.chars().filter(|x| *x == 'J').count();
            if s == 'J' {}
            // let othersnotj = otherchars.chars().filter(|x| *x != 'J').collect::<String>();
            // // check for 3 and 2 = 5
            // // check for 3 = 4
            // // check for 2 2 = 3
            // // check for 2 = 2
            // if countj == 4 {
            //     // full house
            //     if 7 >= score {
            //         score = 7;
            //     }
            // } else if countj == 3 {
            //     // JJJAB
            //     for oc in othersnotj.chars() {
            //         if othersnotj.chars().filter(|x| *x == oc).count() == 1 {
            //             // JJABC
            //             // three of a kind
            //             if 4 >= score {
            //                 score = 4;
            //             }
            //         } else {
            //             panic!("should not get here");
            //         }
            //     }
            // } else if countj == 2 {
            //     // JJABB
            //     // JJABC

            //     for oc in othersnotj.chars() {
            //         if othersnotj.chars().filter(|x| *x == oc).count() == 2 {
            //             // 2 pairs
            //             if 3 >= score {
            //                 score = 3;
            //             }
            //         } else if othersnotj.chars().filter(|x| *x == oc).count() == 1 {
            //             // JJABC
            //             // three of a kind
            //             if 4 >= score {
            //                 score = 4;
            //             }
            //         } else {
            //             panic!("should not get here");
            //         }
            //     }
            // } else if countj == 1 {
            //     // AJ234
            //     // AJ224
            //     // AJ234
            //     // AJ222
            //     // AJ233
            //     for oc in othersnotj.chars() {
            //         if othersnotj.chars().filter(|x| *x == oc).count() == 3 {
            //             // AJ222
            //             // four of a kind
            //             if 6 > score {
            //                 score = 6;
            //             }
            //         } else if othersnotj.chars().filter(|x| *x == oc).count() == 2 {
            //             // AJ233
            //             // two pairs
            //             if 3 > score {
            //                 score = 3;
            //             }
            //         } else {
            //             // AJ234
            //             // one pair
            //             if 2 > score {
            //                 score = 2;
            //             }
            //         }
            //     }
            // }

            if all_unique {
                if 1 >= score {
                    score = 1;
                }
            }
        }
    }
    if score == 0 {
        panic!("score is 0 hand : {}", hand.0);
    }
    println!("hand {:?} score {}", hand, score);
    Some(score)
}

fn main() {
    let input = include_str!("../../inputs/day07.txt");
    // let input = r"32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    let mut items = input
        .lines()
        .map(|x| {
            let items = x.split_whitespace().collect::<Vec<_>>();
            (items[0].to_string(), items[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();
    items.sort_by(|a, b| {
        let hand1 = if PART1 {
            f(a).unwrap()
        } else {
            f3(a.0.chars().collect::<Vec<char>>()).unwrap()
        };
        let hand2 = if PART1 {
            f(b).unwrap()
        } else {
            f3(b.0.chars().collect::<Vec<char>>()).unwrap()
        };
        if hand1 < hand2 {
            return Ordering::Less;
        } else if hand1 > hand2 {
            return Ordering::Greater;
        } else {
            let mut suittopoints = HashMap::new();

            if PART1 {
                suittopoints.insert('A', 13);
                suittopoints.insert('K', 12);
                suittopoints.insert('Q', 11);
                suittopoints.insert('J', 10);
                suittopoints.insert('T', 9);
                suittopoints.insert('9', 8);
                suittopoints.insert('8', 7);
                suittopoints.insert('7', 6);
                suittopoints.insert('6', 5);
                suittopoints.insert('5', 4);
                suittopoints.insert('4', 3);
                suittopoints.insert('3', 2);
                suittopoints.insert('2', 1);
            } else {
                suittopoints.insert('A', 13);
                suittopoints.insert('K', 12);
                suittopoints.insert('Q', 11);
                suittopoints.insert('T', 10);
                suittopoints.insert('9', 9);
                suittopoints.insert('8', 8);
                suittopoints.insert('7', 7);
                suittopoints.insert('6', 6);
                suittopoints.insert('5', 5);
                suittopoints.insert('4', 4);
                suittopoints.insert('3', 3);
                suittopoints.insert('2', 2);
                suittopoints.insert('J', 1);
            }

            for (i, ahand) in a.0.chars().enumerate() {
                let bhand = b.0.chars().nth(i).unwrap();
                if ahand != bhand {
                    let aval = suittopoints.get(&ahand).unwrap();
                    let bval = suittopoints.get(&bhand).unwrap();
                    if aval < bval {
                        return Ordering::Less;
                    } else if aval > bval {
                        return Ordering::Greater;
                    }
                }
            }
            println!("a: {:?} b: {:?}", a, b);
            panic!("should not get here");
        }
    });

    let mut sum = 0;
    for (i, item) in items.iter().enumerate() {
        let rank = i + 1;
        let bid = item.1;
        let val: i64 = (rank as i32 * (bid as i32)) as i64;
        // println!(
        //     "rank {}, {:?} score: {}",
        //     rank,
        //     item,
        //     f3(item.0.chars().collect::<Vec<char>>()).unwrap()
        // );
        sum += val;
    }
    println!("sum: {}", sum);
    cmptest();

    //248968388
    //248615593
    //248569531 good

    //part2
    // checked
    //250562310
    //251177828
    //251177548
    //251097606
    //251444634
    //251444634
    //251428428
    //251509213
    //251509213
    //250003974
    //251721081
    //251721081
    //251640034
    //251420259
    //250340282
    //not checked
    //250382098
}
