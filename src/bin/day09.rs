static PART1: bool = true;
fn gen_deltas(v: Vec<i32>) -> Vec<i32> {
    let mut deltas = Vec::new();
    for i in 0..v.len() - 1 {
        deltas.push(v[i + 1] - v[i]);
    }
    // println!("deltas: {:?}", deltas);
    deltas
}

fn gen_next(v: Vec<i32>) -> i32 {
    let mut delta = gen_deltas(v.clone());
    let mut deltas = vec![delta.clone()];
    let mut not_all_zeros = (&delta).iter().filter(|x| **x != 0).count() != 0;
    while not_all_zeros {
        delta = gen_deltas(delta.clone());
        deltas.push(delta.clone());
        not_all_zeros = (&delta).iter().filter(|x| **x != 0).count() != 0;
    }
    let mut next = 0;
    // println!("deltas len: {}", deltas.len());
    for (i, d) in deltas.iter().rev().skip(1).enumerate() {
        if PART1 {
            // val = next + last
            let newnext = next + d.last().unwrap();
            next = newnext;
        } else {
            // first - next = val
            // val = first - next;
            println!("d: {:?}", d);
            let newnext = d.first().unwrap() - next;
            println!("first: {} - next: {}", d.first().unwrap(), next);
            next = newnext;
        }
        println!("step: {}, next: {}", i, next);
    }
    if PART1 {
        let newnext = next + v.last().unwrap();
        newnext
    } else {
        let newnext = v.first().unwrap() - next;
        newnext
    }
}
fn main() {
    let input = include_str!("../../inputs/day09.txt");
    // let input = r"0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45";

    let steps: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| str::parse::<i32>(x).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for s in steps.iter() {
        let val = gen_next(s.clone());
        sum += val;
        println!("next: {}", val);
    }

    println!("sum = {}", sum);
    // let val = gen_next(steps[2].clone());
    // println!("next: {}", val);
}
