fn main() {
  let s1 = 5;
  let s2 = String::from("5");

  call_stack(s1);
  println!("stack2 {}", s1);

  call_heap(s2);
  println!("heap2 {}", s2);// 异常，函数调用时已经释放了
}

fn call_stack(s: i32) {// copy
  println!("stack1 {}", s);
}// 释放s

fn call_heap(s: String) {// move
  println!("heap1 {}", s);
}// 释放s，释放掉了s2的堆内存


