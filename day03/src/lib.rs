const NUM_BITS: usize = 12;

pub fn part1(numbers: &[u16]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut mask = 1;
    for _ in 0..NUM_BITS {
        let on_count = numbers.iter().filter(|&num| num & mask != 0).count();
        if on_count >= numbers.len() / 2 {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
        mask <<= 1;
    }
    u32::from(gamma) * u32::from(epsilon)
}

fn o2_gen_rating(numbers: &[u16]) -> Option<u16> {
    let mut filtered = numbers.to_vec();
    let mut mask = 1 << (NUM_BITS - 1);
    while filtered.len() > 1 {
        if mask == 0 {
            return None;
        }
        let on_count = filtered.iter().filter(|&num| num & mask != 0).count();
        if 2 * on_count >= filtered.len() {
            filtered.retain(|&num| num & mask != 0);
        } else {
            filtered.retain(|&num| num & mask == 0);
        }
        mask >>= 1;
    }
    filtered.pop()
}

fn co2_scrub_rating(numbers: &[u16]) -> Option<u16> {
    let mut filtered = numbers.to_vec();
    let mut mask = 1 << (NUM_BITS - 1);
    while filtered.len() > 1 {
        if mask == 0 {
            return None;
        }
        let on_count = filtered.iter().filter(|&num| num & mask != 0).count();
        if 2 * on_count >= filtered.len() {
            filtered.retain(|&num| num & mask == 0);
        } else {
            filtered.retain(|&num| num & mask != 0);
        }
        mask >>= 1;
    }
    filtered.pop()
}

pub fn part2(numbers: &[u16]) -> Option<u32> {
    let o2 = o2_gen_rating(numbers)?;
    let co2 = co2_scrub_rating(numbers)?;
    Some(u32::from(o2) * u32::from(co2))
}
