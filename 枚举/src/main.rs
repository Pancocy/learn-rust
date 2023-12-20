//枚举允许你通过列举可能的 成员（variants） 来定义一个类型, 通过枚举出所有可能的值。
#[derive(Debug)]
#[warn(dead_code)]
enum IpAddress{
    v4(u8,u8,u8,u8),
    v6(String)
}
//枚举不仅可以分别定义每个成员的类型，还可以指定具有多个数据的成员的每个数据的类型


enum States{
    CA,
    LA,
    NY,
    DS
}


fn main() {
    let normal = IpAddress::v4(127,0,0,1);

    let new = IpAddress::v6("C4-D3-9F-I8".parse().unwrap());

    println!("Ip4地址：{:?}",normal);
    println!("Ip6地址：{:?}",new);



    /** 枚举与模式匹配( enum & match) **/

    fn find_place(state:States) -> String{
        match state {
            States::CA => String::from("CA"),
            States::DS => String::from("DS"),
            States::LA => String::from("LA"),
            States::NY => String::from("NY")
        }
    }
    let destination = find_place(States::NY);
    println!("i will go {}",destination);
}
