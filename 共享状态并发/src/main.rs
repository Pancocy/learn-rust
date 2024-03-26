
/*
    互斥器mutex在任意时刻只能有一个线程访问，线程在访问数据之前需要获取到mutex的锁，

*/

use std::sync::{Mutex,Arc};
use std::thread;
use std::rc::Rc;

fn main() {
    //1.mutex<T>,
        // let m = Mutex::new(5);
        // {
        //     let mut num = m.lock().unwrap();
        //     *num = 6;
        // }
        // println!("m = {:?}", m)
    //2.在多个线程之间共享mutex<T>,由于m的锁的所有权不能在多个线程之间传递,考虑使用Rc<T>共享锁的所有权
    let m = Arc::new(Mutex::new(0));
    let mut v=vec![];

    for _ in 0..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }
    println!("m = {:?}", *m.lock().unwrap());
    //编译报错，提示我们，mutex的锁的所有权不能安全地在多个线程之间传递，于是引入acr<T>,
}
