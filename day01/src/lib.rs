pub fn part1(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

pub fn part2(input: &[i32]) -> usize {
    input
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|sums| sums[0] < sums[1])
        .count()
}
