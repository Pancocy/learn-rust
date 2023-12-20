//trait：为类型实现可共享的一系列行为(方法)

/* 声明trait */
use std::fmt::{Display, format};

pub trait Summary{
    //
    // fn summarize() -> String;

    //为trait实现默认的行为
    fn summarize(&self) -> String{
        String::from("......")
    }
}
pub struct NewsPaper{
    author:String,
    content:String,
    createTime:String
}
pub struct Tweet{
    title:String,
    content:String,
    rePly:bool,
    reTweet:bool
}
/* 为类型实现trait*/
//语法：impl ... for ... ,在为类型实现trait时，使用与定义trait时相同的行为名可实现对默认行为的重写
impl Summary for NewsPaper{
    //对trait默认行为的重写
    fn summarize(&self) -> String {
        format!("作者：{}，内容：{}，创建时间：{}",self.author,self.content,self.createTime)
    }

    //使用trait的默认行为，置空则使用定义trait时的默认行为

}
impl Summary for Tweet{
    //对trait默认行为的重写
    fn summarize(&self) -> String {
        format!("{},{},{}",self.title,self.content,self.rePly)
    }

    //使用trait的默认行为，置空则使用定义trait时的默认行为

}

/* trait作为参数
        1.impl traitName
        2.trait bounds语法
*/
pub fn notify(item: &impl Summary){
    println!("{}",item.summarize())
}

pub fn notify_bounds<T,U>(item1:T, item2:U) where T: Summary ,U: Summary  {
    println!("trait bounds:{}",item1.summarize());
    println!("trait bounds:{}",item2.summarize());
}


fn main() {
    let news = NewsPaper{
        author: "franklyn".to_string(),
        content: "this is a test paper content".to_string(),
        createTime: "1999-9-10".to_string(),
    };
    println!("news-Sumarry:{}",news.summarize());

    let tweet = Tweet{
        title: "SpaceX".to_string(),
        content: "this is a test Tewwt".to_string(),
        rePly: false,
        reTweet: false,
    };
    println!("tweet-Sumarry:{}",tweet.summarize());

    /*
            trait作为参数：1.impl traitName
            通过传入任何实例化后的trait作为参数，在nitify方法中调用每个实例的行为
    */
    notify( &news);
    notify(&tweet);

    /*
        trait作为参数：2.trait bounds
    */
    notify_bounds(news,tweet);


}
