fn total_distance_simple(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|&pos| (pos - target).abs()).sum()
}

fn total_distance_incremental(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|&pos| {
            let dist = (pos - target).abs();
            (dist * dist + dist) / 2
        })
        .sum()
}

pub fn part1(positions: &[i32]) -> Option<i32> {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|num| total_distance_simple(positions, num))
        .min()
}

pub fn part2(positions: &[i32]) -> Option<i32> {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|num| total_distance_incremental(positions, num))
        .min()
}
