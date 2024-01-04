//Rc<T>,引用计数智能指针，用以在多线程环境中共享数据，Rc<T>可以理解为T类型的指针，Rc<T>可以有多个指针指向同一个数据
//使用场景：见根目录下图片root.svg
use std::rc::Rc;
fn main() {
    enum List{
        Cons(i32,Rc<List>),
        Nil,
    }
    //b,c同时实现了对象a的引用，当a被drop时，b和c也会被drop
    let a = Rc::new(List::Cons(5,Rc::new(List::Cons(10,Rc::new(List::Nil)))));
    println!("strong_count:{}",Rc::strong_count(&a));
    //drop(a);

    let b = Rc::new(List::Cons(3,Rc::clone(&a)));
    println!("strong_count:{}",Rc::strong_count(&a));

    {
        let c = Rc::new(List::Cons(4,Rc::clone(&a)));
        println!("strong_count:{}",Rc::strong_count(&a));
    }

    println!("strong_count:{}",Rc::strong_count(&a));
    //strong_count()方法返回Rc<T>中引用的数量，克隆 Rc<T> 会增加引用计数,
    //当c离开作用域时(被drop时)，Rc<T>中的引用计数会减1
    // Rc<T>中引用的数量为0时，Rc<T>会被drop
}
