/*
    构建一个与文件和命令行输入/输出交互的命令行工具。
        1.获取命令行的参数，并收集保存至vector中。通过标准库的env的args方法，该方法返回一个迭代器，通过collect收集到vector中。
        2.读取文件。使用标准库的fs模块的read_to_string方法接收file_name，来返回Result格式的文件内容。
 */

use std::{env, process};
use minigrep::Query;
use minigrep::run;

fn main() {
    let args:Vec<String> = env::args().collect();

    //println!("{:?}",args);          //程序名会占据vector的第一个位置
    /*
        let key_words = &args[1];
        let file_name = &args[2];
        println!("{},{}",key_words,file_name);
     */

    let query = Query::config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = run(query) {
        println!("Application error: {:?}", e);
        process::exit(1);
    }
    /*
        let content = fs::read_to_string(file_name).expect("读取文件发生错误！");
        println!("文件内容：\n{}",content);
     */


}
