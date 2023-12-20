use::std::io::stdin;
use::rand::thread_rng;
use rand::Rng;
use::std::cmp::Ordering;

fn main(){
    println!("-------------猜数游戏--------------");
    let secret = thread_rng().gen_range(0..100);

    loop{
        println!("请输入一个数字以开始猜测");
        let mut guess= String::new();
        let  guess = stdin().read_line(&mut guess).expect("读取行失败！！！");

        let guess:u32 = match guess.trim().parse(){
            Ok(guess) => guess,
            Err(_)=>{
                println!("你输入的不是数字");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Greater => println!("太大了！"),
            Ordering::Less => println!("太小了！"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            },

        }

    }
}