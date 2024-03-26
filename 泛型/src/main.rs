/** 泛型是具体类型或其他属性的抽象替代，用于高效处理重复概念的工具。**/

//1.在struct定义中使用泛型,单一泛型指定相同类型，使用多个泛型可指定不同类型
#[derive(Debug)]
struct Rectangle<T,F> {
    width:T,
    height:F
}
//2.在枚举定义中使用泛型
enum Option<T>{
    Some(T),
    None
}
enum Coin<T>{
    Penny(T),
    Dollor(T),
    Thousand(T),
    Million(T)
}
//3.在为struct和enum实现方法时同样可以使用泛型
impl<T,F> Rectangle<T,F>{
    fn mix<U,W>(self,other:Rectangle<U,W>) -> Rectangle<T,W>{
        Rectangle{
            width:self.width,
            height:other.height
        }
    }
}
fn main() {
    //1.泛型可用于提取重复方法

    //2.struct
    let cotan = Rectangle{
        width:10,
        height:1.0
    };
    println!("{:?}",cotan);

    //3.enum
    let penny = Coin::Penny(3);
    let dollor = Coin::Dollor(100);
    let thousand = Coin::Thousand(1000);
    let million = Coin::Million(1000000);

    //4.方法
    let point1 = Rectangle{
        width:String::from("xxz"),
        height:20
    };
    let point2 = Rectangle{
        width:5.20,
        height:String::from("xka")
    };
    let point3 =Point1.mix(point2);
    println!("{:?}",point3);

}
