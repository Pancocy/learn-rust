
use rand;
use rand::Rng;

pub fn add_one(x:i32)->i32{
    x + 1
}

pub fn generate_rand() -> i32{
    rand::thread_rng().gen_range(0..100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
