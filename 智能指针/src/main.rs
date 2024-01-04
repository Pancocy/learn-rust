/*
智能指针（smart pointers）是一类数据结构，它们的表现类似指针，但是也拥有额外的元数据和功能。
    智能指针通常使用结构体实现。
    智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。
    Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码。
    Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。
*/


enum List{
    Cons(i32,List),
    Nil
}

enum NewList{
    Cons(i32,Box<NewList>),
    Nil
}

enum Message{
    Quit,
    Text(String),
    Nums(i32,i32)
}

use crate::List::{Cons,Nil};
fn main() {
    //1.Box指针，允许在堆上存储数据，当Box离开作用域时，将同时作用于栈和堆上
        let i = Box::new(23);
        println!("value = {}",i);

    //2.Box允许使用递归创建编译时位置大小的数据量
    let list = Box::new(Cons(1,Cons(2,Cons(3,Cons(4,Nil)))));

    //3.Box计算非递归枚举的存储空间.计算非递归的枚举类型时，因枚举一次只能使用一个成员，因此Box会计算最大成员所需的存储空间作为box指针的大小
    let num = Box::new(Message::Nums(1,2));

    //4.Box对递归类型的优化，Box此时只需要存储指针的大小，而存储指针的大小则是固定的
    let new_list = Box::new(NewList::Cons(1,Box::new(NewList::Cons(2,Box::new(NewList::Nil)))));

    //实现deref和drop trait,指针可以被解引用，解引用后可以得到指针指向的值，
    let x = 5;
    let y = &5;
    assert_eq!(5,*y);

    let z = Box::new(20);
    let g = &z;
    assert_eq!(20,*g);

    //使用自定义的指针，并实现Deref和Drop trait
    struct MyBox<T>(T);

    impl <T> MyBox<T>{
        fn new(x:T) -> MyBox<T>{
            MyBox(x)
        }
    }

    let boxed_int = MyBox::new(5);
    //assert_eq!(5,*boxed_int);  //报错，因为MyBox没有实现Deref trait

    use std::ops::Deref;
    //为自定义指针myBox实现Deref trait
    // `Deref` trait的实现，用于自定义`MyBox`类型的`deref`方法
    impl <T> Deref for MyBox<T>{
        // 指定目标类型为T
        type Target = T;
        // 定义`deref`方法，返回类型为T类型的引用
        fn deref(&self) -> &T{
            // 返回`self`中的数据部分的引用
            &self.0
        }
    }
    assert_eq!(5,*boxed_int);

    //drop trait
    struct CustomSmartPointer{
        data:String,
    }

    impl Drop for CustomSmartPointer{
        fn drop(&mut self){
            println!("Dropping CustomSmartPointer with data `{}`",self.data);
        }
    }

    let a = CustomSmartPointer{data:String::from("my data")};
    let b = CustomSmartPointer{data:String::from("other data")};
    //当程序编译时，当它们离开作用域时，会调用drop方法，打印出数据，先打印b，再打印a
}
