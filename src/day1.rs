use std::collections::HashSet;

pub fn solve(content: String) {
  let mut map = HashSet::<i32>::new();
  let lines = content.lines();
  for line in lines {
    let a: i32 = line.trim().parse().expect("not a number");
    let _lines = content.lines();
    for lin in _lines {
      let b: i32 = lin.trim().parse().expect("not a number");
      let c = 2020 - a - b;
      if map.contains(&c) {
        //this is the pair
        println!("{} * {} * {} = {}", a, b, c, a * b * c);
      } else {
        map.insert(a);
      }
    }
  }
}
