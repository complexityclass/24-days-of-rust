use primal::Sieve;
use std::collections::HashSet;
use num::integer::Roots;

struct NaiveSieve {
    set: HashSet<usize>
}

impl NaiveSieve {
    fn new(size: usize) -> Self {
        let mut sieve = NaiveSieve { set: HashSet::with_capacity(size) };
        sieve.inhabit(size);
        sieve
    }

    fn is_prime(&self, n: usize) -> bool {
        self.set.contains(&n)
    }

    fn inhabit(&mut self, n: usize) {
        for i in 2..n {
            self.set.insert(i);
        }
        for candidate in 2..((n as i32).sqrt() as usize) {
            let mut to_remove: Vec<usize> = Vec::new();
            for elem in &self.set {
                if *elem != candidate && elem % candidate == 0 {
                    to_remove.push(*elem);
                }
            }

            for r in to_remove {
                self.set.remove(&r);
            }
        }
    } 
}

fn main() {
    let size = 10000 as usize;
    let naive_sieve = NaiveSieve::new(size);
    let sieve = Sieve::new(size);

    for suspect in 0..1000 {
        let expected = sieve.is_prime(suspect as usize);
        let actual = naive_sieve.is_prime(suspect as usize);
        if expected != actual {
            panic!("wrong impl on {}", suspect);
        }
        if actual {
            println!("{} is primal!", suspect);
        }
    }
    println!("Done checking");
}