fn main() {
    let races = [
        Race::new(42, 308),
        Race::new(89, 1170),
        Race::new(91, 1291),
        Race::new(89, 1467),
    ];

    let part1: usize = races.iter().map(|r| r.possibilities()).product();
    println!("Part 1: {}", part1);

    println!(
        "Part 2: {}",
        Race::new(42_89_91_89, 308_1170_1291_1467).possibilities()
    );
}

struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn new(time: usize, record: usize) -> Self {
        Self { time, record }
    }
    fn sim(&self, time_press: usize) -> usize {
        let time_left = self.time - time_press;
        time_left * time_press
    }

    fn possibilities(&self) -> usize {
        let mut possibilities = 0;
        for i in 0..=self.time {
            if self.sim(i) > self.record {
                possibilities += 1;
            } else if possibilities > 0 {
                break;
            }
        }

        possibilities
    }
}
