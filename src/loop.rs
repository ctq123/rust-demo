fn main() {
  let mut x = 3;

  let result = loop {
    println!("x {}", x);
    x -= 1;
    if x == 0 {
      break x + 2;// 这里的返回值是带分号的
    }
  };
  println!("result: {}", result);// 2

  let mut y = 3;
  while y != 0 {
    println!("y {}", y);
    y -= 1;
  }

  let z = [1, 2, 3];
  for e in z.iter() {
    println!("z {}", e);
  }
}
