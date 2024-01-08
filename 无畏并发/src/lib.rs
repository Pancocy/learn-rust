
//创建一个新通道，rust中允许有多个发送者，但是只能有一个接收者
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn create_channel(){
    let (tx,rx) = mpsc::channel();
    //创建线程,并发送消息
    thread::spawn(move || {
        let msg = String::from("hello world");
        tx.send(msg).unwrap();
        //在发送完消息后，尝试访问msg;会发现msg发生了移动
        //println!("Sent {} on thread",msg);
    });
    //在主线程中接收并打印
    let message = rx.recv().unwrap();
    println!("Got from tr {}",message);
}

pub fn  mutiple_send(){
    //通过克隆实现多个生产者，发送多个信息
    let (tx,rx) = mpsc::channel();

    let tx_copy = tx.clone();
    thread::spawn(move||{
        let vec1 = vec![
            String::from("xxz"),
            String::from("xka")
        ];
        for val in vec1{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move||{
        let vec2 = vec![
            String::from("hello"),
            String::from("panpan")
        ];
        for val in vec2{
            tx_copy.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //取消使用rx的recv方法，改用迭代器
    for received in rx{
        println!("Got {}",received);
    }
}
