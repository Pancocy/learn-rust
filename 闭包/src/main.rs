use std::thread;
use std::time::Duration;

pub fn generate_plan(intensity:u32,random:u32){
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
}

fn main() {

}

#[cfg(test)]
    mod test{
    use super::*;

    #[test]
    fn it_works(){
        let res = generate_plan(35,5);
    }
}
