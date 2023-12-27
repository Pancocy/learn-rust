/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let mut num_vec = vec![1,2,3];
/// let ev= super::every_add(num_vec,5);
/// assert_eq!(
///     ev,
///     vec![6,7,8],
///     "测试失败！！！"
/// )
/// ```

pub fn main() {
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
}
//迭代器的trait中定义了一些可以生成其他不同类型的迭代器的方法，称为迭代器适配器，如：map、filter

pub fn every_add(vec: Vec<i32>,target:i32) -> Vec<i32>{
    let res = vec.into_iter().map(|x| x + target).collect();
    res
}

#[derive(PartialEq, Debug)]
pub struct shoe{
    size:i32,
    style:String
}
pub fn collect_same_size_boot(vec:Vec<shoe>,size:i32) -> Vec<shoe>{
    let res = vec.into_iter().filter(|shoe| shoe.size == size).collect();
    res
}
#[cfg(test)]
mod tests{
    pub use super::*;
    #[test]
    fn add_every(){
        let mut num_vec = vec![1,2,3];
        let ev= super::every_add(num_vec,5);
        assert_eq!(
            ev,
            vec![6,7,8],
            "测试失败！！！"
        )
    }

    #[test]
    fn find_shoes(){
        let shoes = vec![
            shoe {size:10,style:String::from("snowBoot")},
            shoe {size:12,style:String::from("runBoot")},
            shoe {size:10,style:String::from("freeBoot")},
        ];
        let es = super::collect_same_size_boot(shoes,10);
        assert_eq!(
            es,
            vec![
                shoe {size:10,style:String::from("snowBoot")},
                shoe {size:10,style:String::from("freeBoot")}
            ],
            "测试失败！！！"
        )
    }

}
