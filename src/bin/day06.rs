fn ways_to_win(n: (u64, u64)) -> u64 {
    let mut ways = 0;
    let distance_to_beat = n.1;
    for i in 0..n.0 {
        let speed = i;
        let time = n.0 - i;
        let distance = speed * time;
        if distance > distance_to_beat {
            ways += 1;
        }
    }
    ways
}
fn main() {
    // part1
    let times = vec![(46, 208), (85, 1412), (75, 1257), (82, 1410)];

    // part2
    let times = vec![(46857582, 208141212571410)];
    let total_ways = times
        .iter()
        .map(|x| ways_to_win(*x))
        .collect::<Vec<_>>()
        .iter()
        .fold(1, |acc, x| acc * x);
    println!("total ways: {}", total_ways);
}
