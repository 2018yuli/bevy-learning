fn main() {
    /*
    默认做复制所有权的操作的有 7 种。
        所有的整数类型，比如 u32:
        布尔类型 bool;
        浮点数类型，比如 f32、f64;
        字符类型 char;
        由以上类型组成的元组类型 tuple，如(i32,i32,char);
        由以上类型组成的数组类型 array，如 [9; 100]；
        不可变引用类型 &。

    其他类型默认都是做移动所有权的操作
     */
    let s = "I am a superman!".to_string();
    for i in 1..10 {
        // 编译报错 let tmp_s = s;
        println!("s is {s}");
    }
}
