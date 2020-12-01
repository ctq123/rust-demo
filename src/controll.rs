fn main() {
  let number = 3;

  // 正常
  if number < 5 {
      println!("condition was true");
  } else {
      println!("condition was false");
  }

  // 异常
  if number {// 会抛出异常，数值不会自动转换为bool
    println!("number is 3");
  }

  // 异常
  let x = if true { 5 } else { "six" };// 抛出异常，因为x无法获得准确的类型（数值/字符串）

}
