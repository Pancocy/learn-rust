mod main_copy;

use std::io;
use rand::Rng;
use::std::cmp::Ordering;
fn main() {
    println!("--------猜数游戏----------");
    let  target = rand::thread_rng().gen_range(0..100);
    loop {
        println!("请输入1~100之间的数字：");
        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("读取行失败");
        let num: u32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("输入不是数字");
                continue;
            }
        };

        match num.cmp(&target) {
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            },
            Ordering::Greater => println!("太大了！"),
            Ordering::Less => println!("太小了！")
        }



    }
}
