use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct  Query{
    pub key_words:String,
    pub file_name:String
}

impl Query{
    pub fn config(args:&[String]) -> Result<Query,&'static str>{
        //对用户不输入参数的情况做错误处理
        if args.len() < 3{
            return Err("参数不足！")
        }
        let key_words = args[1].clone();
        let file_name = args[2].clone();
        Ok(Query{key_words,file_name})
    }
}

pub fn run(query: Query) -> Result<(),Box< dyn Error > >{
    let content = fs::read_to_string(query.file_name)?;

    for line in search(&query.key_words, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(keys: &str, context:&'a String) ->Vec<&'a str>{
    let mut vec1 = vec![];
    for line in context.lines(){
        if line.contains(keys){
            vec1.insert(0,line)
        }
    }
    vec1
}


//编写自动化测试，将项目所有的提示性错误信息去除，因为，我们要在测试中实现这些信息。
#[cfg(test)]
    mod test{
    use super::*;
    //失败的测试
    #[test]
    fn failure_work(){
        let query = "xxzxka";
        let contents = "\
Rust:.to_string()
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, &contents.to_string()), // []
            "目标短语中不含该关键字"
        );
    }

    //编写成功的测试
    #[test]
    fn success_work(){
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, &contents.to_string()) // []
        );
    }
}