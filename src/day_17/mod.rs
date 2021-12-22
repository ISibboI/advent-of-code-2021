use std::ops::RangeInclusive;

static INPUT_X: RangeInclusive<i64> = 150..=193;
static INPUT_Y: RangeInclusive<i64> = -136..=-86;

pub fn problem_1() {
    println!("Day 17 problem 1: {}", (0..=135).sum::<u64>());
}

pub fn problem_2() {
    fn simulate(mut dx: i64, mut dy: i64) -> Option<i64> {
        let mut x = 0;
        let mut y = 0;
        let mut max_y = 0;

        while y >= *INPUT_Y.start() {
            x += dx;
            dx -= dx.signum();
            y += dy;
            dy -= 1;
            max_y = max_y.max(y);

            if INPUT_Y.contains(&y) && INPUT_X.contains(&x) {
                return Some(max_y);
            }
        }

        None
    }

    let mut result = 0;

    for dx in 0..194 {
        for dy in -136..137 {
            if simulate(dx, dy).is_some() {
                result += 1;
            }
        }
    }

    println!("Day 17 problem 2: {}", result);
}
