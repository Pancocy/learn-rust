//struct是一个自定义数据类型，允许你命名和包装多个相关的值，从而形成一个有意义的组合
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    //单个实现内定义多个方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn contain(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

//struct多个实现
impl Rectangle {
    fn same_one(&self,other:&Rectangle) -> bool{
        self.height == other.height && self.width ==other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 15,
    };

    println!("rect1的面积:{}", rect1.area());

    let rect2 = Rectangle {
        width: 13,
        height: 20,
    };

    println!("rect2是否能容纳rect1:{}", rect2.contain(&rect1));
    println!("rect1是否能容纳rect2:{}", rect1.contain(&rect2));

    println!("rect1和rect2是否相同:{}", rect1.same_one(&rect2));
}
