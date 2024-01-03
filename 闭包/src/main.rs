use std::thread;
use std::time::Duration;

//使用具有泛型和Fn trait的闭包,
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: T,
            value: None,
        }
    }
    fn value(&mut self, args: u32) {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args);
                // self.calculation是一个Fn trait，|intensity|{intensity}，传入args作为intensity，返回args的值，赋值给self.value
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_plan(intensity: u32, random: u32) {
    /* 普通版闭包
    let get_intensity = |intensity| {
        println!("please wait,calculating......");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    let ger_intensity = get_intensity(intensity);
    if intensity < 30{
        println!("run {:?} km",ger_intensity);
        println!("do {:?} push_ups",ger_intensity);
    }else{
        if random == 3{
            println!("take a breaking today!")
        }else{
            println!("redo after {} days!",random)
        }
    }
     */
    /*
        使用具有泛型的闭包
    */
    //1.new方法传递一个实现闭包的fn trait：|intensity|{},返回一个Cacher结构体，其中calculation为|intensity|{}，value为None；
    let get_intensity = Cacher::new(|intensity| {
        println!("please wait,calculating......");
        thread::sleep(Duration::from_secs(2));
        intensity
    }); // Cacher {calculation:|intensity|{intensity} ,value:None}
    if intensity < 30 {
        //通过调用第一步返回的结构体本身的value方法，传递第一个参数为结构体本省的calculation，第二个参数为用户输入的intensity
        println!("run {:?} km", get_intensity.value(intensity));
        println!("do {:?} push_ups", get_intensity.value(intensity));
    } else {
        if random == 3 {
            println!("take a breaking today!")
        } else {
            println!("redo after {} days!", random)
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let res = generate_plan(35, 5);
    }
}
