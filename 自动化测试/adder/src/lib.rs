use std::fmt::format;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn notify(name:&str) -> String{
    //通过的示例
    format!("Hello,{}",name)
    // 错误的示例
    //String::from("Hello,")
}

pub struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    pub fn can_hold(&self,another:Rectangle) -> bool{
        self.width >= another.width && self.height >= another.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    //基础测试
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    //自定义错误信息的测试：
    #[test]
    fn test_notify(){
        let result = notify("xxz");
        assert!(
            result.contains("xxz"),
            "notify not contain name,as value:{}",result
        )
    }
    //使用assert！宏实现测试，assert！接受bool参数，当为true时什么也不做，当为false时，assert！会调用panic！，道中测试不通过
    #[test]
    fn test_can_hold(){
        let rect1 = Rectangle{
            width:20,
            height:120
        };
        let rect2 = Rectangle{
            width:30,
            height:120
        };
        let flag = rect2.can_hold(rect1);
        assert!(
            flag,
            "rect2不能容纳rect1，flag is {}",flag
        )
    }
}

/*
assert! , assert_eq!,sert_ne!宏，第二个参数都会传递给format宏，当测试不通过时，作为自定义错误信息输出
*/

/*
        1.使用#[ignore]忽略特定测试
        2.rust的测试时通过线程控制的，默认状态下使用：cargo test将会 *并行* 运行所有测试，
        但当出现测试之间存在依赖情况时，可能导致测试结果不准确。使用：cargo test -- --test-threads=线程数来控制并行数量。
        3.显示函数输出，当目标测试函数存在输出时，成功通过测试的函数输出将被截获而不被显示，测试失败则会显示，想要通过时显示输出，
        可使用：cargo test -- --nocapture
 */