pub struct PrimeSieve {
    primes: Vec<u64>,
}

impl PrimeSieve {
    pub fn new() -> Self {
        PrimeSieve { primes: Vec::new() }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        // Hardcode the first primes, so we only have to consider odd numbers later on
        match self.primes.len() {
            0 => {
                self.primes.push(2);
                return Some(2);
            }
            1 => {
                self.primes.push(3);
                return Some(3);
            }
            _ => { /* See below */ }
        }
        let mut n = *self.primes.last().unwrap() + 2;
        let limit = f64::sqrt(n as f64).ceil() as u64;
        loop {
            let mut is_prime = true;
            for p in self.primes.iter() {
                if n % p == 0 {
                    is_prime = false;
                    break;
                } else if *p > limit {
                    break;
                }
            }
            if is_prime {
                self.primes.push(n);
                return Some(n);
            }
            n += 2;
        }
    }
}
