use std::thread;
use std::time::Duration;
use fearless::{create_channel, mutiple_send};
fn main() {
        // 1. 创建一个线程
        let handle = thread::spawn(||{
            for i in 1..10{
                println!("{},form new thread!",i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // 2. 在主线程中执行代码
        for i in 1..5{
            println!("{},from main thread!",{i});
            thread::sleep(Duration::from_millis(1));
        }

        // 3. 等待线程结束,join方法会阻塞主线程,直到子线程结束
        handle.join().unwrap();

        //move闭包会使线程获得值的所有权,
        let v = vec![1,2,3];
        thread::spawn(move || {
            println!("move v:{:?}",v);
        }).join().unwrap();

        //drop(v);//使用move闭包后，v被移动至子线程中，在主线程中v将不可用

        //通道
        create_channel();

        mutiple_send();
}
