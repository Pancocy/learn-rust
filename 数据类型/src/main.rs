use::std::io;

fn main(){
    //通过mut声明可变与不可变，,let+mut 声明可变变量，let声明为不可变变量
    let x = 200;
    println!("{}",x); // 200
    //x = 100 ,报错

    let mut y = 100 ;
    y = 200;
    println!("{}",y);

    //const声明常量,始终不可变；声明值的类型必须注明
    const Z: i32 = 100;

    let tup = (0.3,32,true,'c');
    let (a,b,c,d) = tup;
    println!("tup:{:?}",tup.0);

    let arr = [1,2,3,4,5 ];
    println!("{}",arr[0]);

    println!("请输入索引");

    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("读取行错误");
    let index:usize = index.trim().parse().expect("输入不是数字");


    let element = arr[index];

    println!("对应索引的值是:{}",element);
}
