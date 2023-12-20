
//rust核心只有一种字符串类型:str，&str是字符串的切片(slice),是对str的借用；str不能通过索引访问
fn main() {
    //创建字符串的方法：String::new()、String::from、to_string();

    //1.String::new()
        let mut str = String::new();        //创建一个空字符串
        str.push('x');                         //push方法只能指定char类型进行添加
        str.push_str("xzxka");              //push_str可添加字符串字面量
        println!("{}",str);

    //2.String::from
        let xer = String::from("xxzxka");
        println!("{}",xer);

    //3.to_string()
        let gtr = "initial";
        println!("{}",gtr.to_string());


    //使用+运算符或者format宏拼接字符串
        println!("{}",str.clone() + "-" +gtr + "-" + &xer);
        println!("{}",format!("{}-{}-{}",str ,gtr,xer));

    //遍历字符串：for循环，str.chars()、str.bytes()
        for s in xer.chars(){
                println!("{}",s)      //返回每一个字符char
        }
        for k in xer.bytes(){
                println!("{}",k)      //返回每一个字符的字节
        }
    //字符串slice
        let part = &xer[0..2];
        println!("{}",part);
}
