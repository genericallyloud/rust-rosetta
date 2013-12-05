// Implements http://rosettacode.org/wiki/Hailstone_sequence
struct Hailstone {
  priv current: int,
       start:   int
}

impl Hailstone {
  fn new(n: int) -> Hailstone {
    Hailstone { current: n, start: n }
  }
}

impl Iterator<int> for Hailstone {
  fn next(&mut self) -> Option<int> {
    let current = self.current;
    match current {
      0               => {
        self.current = self.start; // Resets the iterator.
        None
      },
      1               => {
        self.current = 0;
        Some(1)
      },
      x if x % 2 == 0 => {
        self.current = x / 2;
        Some(current)
      },
      x if x % 2 == 1 => {
        self.current = (3 * x) + 1;
        Some(current)
      },
      _               => None
    }
  }
}

fn main() {
  // Find the hailstone for 27.
  let two_seven: ~[int] = Hailstone::new(27).collect();
  println!("Testing: {}, Length: {}, Values: {}...{}", two_seven[0], two_seven.len(), two_seven.slice(0,4).to_str(), two_seven.slice(two_seven.len()-4, two_seven.len()).to_str());
  // Find the longest.
  let mut biggest = range(0, 100000).map(|x| Hailstone::new(x)).max_by(|&mut x| x.len()).unwrap();
  let size = biggest.len();
  println!("Largest: {}, Size: {}", biggest.start, size);
}
