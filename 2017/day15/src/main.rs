fn main() {
    let gen_a = Generator::new(703, 16807);
    let gen_b = Generator::new(516, 48271);

    println!("Answer #1: {}", judge(gen_a, gen_b, 40000000));
}

fn judge(generator_a: Generator, generator_b: Generator, iterations: usize) -> usize {

    let mut gen_a = generator_a.into_iter();
    let mut gen_b = generator_b.into_iter();

    let mut total_matches = 0;

    for _ in 0..iterations {
        if gen_a.next() == gen_b.next() {
            total_matches += 1
        }
    }

    total_matches
}

#[derive(Debug)]
struct Generator {
    state: usize,
    factor: usize,
}

impl Generator {
    fn new(start: usize, factor: usize) -> Generator {
        Generator {
            state: start,
            factor: factor,
        }
    }
}

impl Iterator for Generator {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        const MOD: usize = 2147483647;
        let next = (self.state * self.factor) % MOD;
        self.state = next;

        Some(self.state as i16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_judge_matches_small() {
        let gen_a = Generator::new(65, 16807);
        let gen_b = Generator::new(8921, 48271);

        assert_eq!(1, judge(gen_a, gen_b, 5));
    }
    #[test]
    fn test_judge_matches_large() {
        let gen_a = Generator::new(65, 16807);
        let gen_b = Generator::new(8921, 48271);

        assert_eq!(588, judge(gen_a, gen_b, 40000000));
    }
}
