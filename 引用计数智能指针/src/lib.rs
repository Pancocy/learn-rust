//RefCell<T>和内部可变性：对不可变值的可变引用
//Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
//Box<T>在编译时可对值进行可变和不可变引用，Rc<T>只能在编译时对值进行不可变引用，RefCell<T>可以在运行时进行可变和不可变引用。
pub fn suit_refcell() {
    // let x = 5;
    // let y = &mut x;


    let mut x = 5;
    let y = &mut x;
    println!("x = {},y= {}", x,y); // 无法对同一个值同时进行可变引用和不可变引用
}
