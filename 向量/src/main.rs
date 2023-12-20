//常见的集合：vector,string,hashMap

#[derive(Debug)]
enum Coin{
    Penny(f32),
    Cent(u8),
    Dollor(u16)
}

fn main() {
//vec的子项应具有相同的类型
    /*
    创建vec的方式
        1.Vec::new()
        2.vec![]
    */
    let mut vec1 = Vec::new();
    println!("{:?}",vec1);

    let mut vec2 = vec![0.2, 4.2, 9.0];
    println!("{:?}",vec2);

//获取向量：通过 索引 或者 get方法(get方法返回的是一个some类型的数据)；使用超出vector的索引访问时，索引法将会导致程序panic，而get方法则不会，而是返回none；
    println!("通过索引获取：{:?}",vec2[0]);

    let x = vec2.get(2);
    println!("通过get方法获取：{:?}",x);

//更新向量
    vec1.push(2);
    println!("更新后：{:?}",vec1);

//遍历vector：可以通过for循环便利vector,可以在遍历时对vector的 可变引用 进行赋值，使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
    for k in &mut vec2{
        *k += 0.5;
        println!("{}",k);
    }

//一般来说vector中只能存储相同类型的数据，但是可以通过枚举实现存储不同类型
    let vec_enum = vec![
        Coin::Penny(0.5),
        Coin::Cent(3),
        Coin::Dollor(100)
    ];
    println!("通过枚举定义存储不同类型的vector:{:?}",vec_enum)
}
//vector 在其离开作用域时会被释放
