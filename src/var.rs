fn main() {
  let x = 5;

  let y = {
    let x = 3;
    x + 1;
  };

  // 异常
  // let y = {
  //   let x = 3;
  //   x + 1;
  // };

  println!("The value of x is: {}, y is: {}", x, y);
}