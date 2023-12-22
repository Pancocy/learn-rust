fn main() {
    let v = vec![1,2,3,4,5];
    let i = v.iter();
    for &k in i{
        println!("{}",k);
    }
    //使用for循环遍历迭代器时不要求迭代器可变，因为for获取了它的所有权，而直接在迭代器上调用next方法则需要迭代器可变，因为每一个 next 调用都会从迭代器中消费一个项
    let mut i2 = v.iter();
    assert_eq!(Some(&1),i2.next());

    let sum:i32 = i2.sum(); //sum是迭代器的一个消费适配器，在调用sum后迭代器将不再可用，因为sum获取了它的迭代器
    println!("{}",sum);

    //assert_eq!(Some(&1),i2.next())


    //迭代器的trait中定义了一些可以生成其他不同类型的迭代器的方法，称为迭代器适配器，如：map、filter
}
