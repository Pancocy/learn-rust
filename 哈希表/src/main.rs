use std::collections::HashMap;

//hashMap：HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射；可以用于需要 **任何类型作为键** 来寻找数据的情况。
fn main() {
    //1.创建hashMap
        let mut hash = HashMap::new();

    //2.获取hashMap的数据： insert(key,value)方法、
        hash.insert(String::from("green"),String::from("tree"));
        hash.insert(String::from("xxz"),String::from("小可爱"));
        hash.insert(String::from("papan"),String::from("大可爱"));
    //3.获取hashMap的数据： get(key)方法、(返回Some类型数据)
        let s = hash.get("papan");
        println!("{:?}",hash);
        println!("{:?}",s);
    //4.更新HashMap：可通过insert相同的key以实现value的覆盖;   entry...or_insert:仅当无匹配key时才插入数值。
        hash.insert(String::from("green"),String::from("leafs"));
        hash.entry(String::from("blue")).or_insert(String::from("great color"));
    //5.遍历HashMap
        for (k,v) in &hash{
                println!("key:{},value:{}",k,v);
        }
}
