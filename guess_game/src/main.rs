use std::io;// 表示输入/输出，与C++的语法类似
use std::cmp::Ordering;
use rand::Rng;// 需要在Cargo.toml的dependencies添加rand = "0.5.5"

fn main() {
    println!("Let's guess the number!");
    let target_number = rand::thread_rng().gen_range(1, 101);// 表示[1-101),包含下限，不包括上限，即1-100之间的数字
    
    loop {// while循环
        println!("Please input your number:");

        let mut num_str = String::new();// mut表示可变的，mutable

        io::stdin()
            .read_line(&mut num_str)
            .expect("Failed to read line");// stdin表示输入，read_line读取一行数据，&表示引用，类似C++

        let num_str: u32 = match num_str.trim().parse() {// 对输入的数字进一步处理，忽略无效输入
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", num_str);

        match num_str.cmp(&target_number) {// 比较
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;// 跳出循环
            }
        }
    }
}
