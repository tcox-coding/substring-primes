use std::collections::HashSet;
use std::io::{self};

fn get_all_substrings(s: &str) -> Vec<&str> {
  // Collect all valid char boundary byte indices, plus the ending index
  let indices: Vec<usize> = s.char_indices().map(|(i, _)| i).chain(std::iter::once(s.len())).collect();
  let mut substrings = Vec::new();
  
  // Generate all combinations of start and end indices
  for i in 0..indices.len() {
    for j in (i + 1)..indices.len() {
      let start = indices[i];
      let end = indices[j];
      substrings.push(&s[start..end]);
    }
  }
  substrings
}

fn main() -> io::Result<()> {
  let content = std::fs::read_to_string("all_primes_lt_1m.txt")?;
  
  let mut primes: HashSet<u64> = HashSet::new();
  
  // Construct the hash-set of primes.
  for prime in content.lines() {
    let p = prime.parse::<u64>().unwrap();
    primes.insert(p);
  }
  
  // Next, check if prime is a "substring prime", i.e. all substrings possible of the prime are also prime.
  let mut substring_primes: Vec<&str> = vec![];
  for pstring in content.lines() {
    let substrings: Vec<&str> = get_all_substrings(pstring);

    // Now that we have substrings, check if each substring is in primes HashSet.
    // Since possible substring primes are less than the length of the string, we are unconcerned with the fact the list only goes to 1m.
    // "Only checked the first million primes".
    let mut has_failed: bool = false;
    for substring in substrings {
      let possible_prime = substring.parse::<u64>().unwrap();
      if !primes.contains(&possible_prime) {
        has_failed = true;
        break;
      }
    }

    // Valid "substring prime"; add to Vec of substring primes.
    if !has_failed {
      substring_primes.push(pstring);
    }
  }
  
  println!("{:#?}", substring_primes);

  Ok(())
}