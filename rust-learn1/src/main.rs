use std::result;

// rust 安装 curl --proto '=https'--tlsv1.2 -sSf https://sh.rustup.rs | sh
// 在线 https://play.rust-lang.org/
// windows 中，不想安装 visual studio，想使用 gnu 工具链：rustup default stable-gnu
// cargo fmt
// cargo clippy
// rustc -V
// cargo build
// cargo run
fn main() {
    // isize 和 usize 的位数与具体 CPU 架构位数有关。CPU 是 64 位的，它们就是 64 位
    let u_int: u32 = 1;
    println!("Hello, world!{}", u_int.to_string());
    /*
    1. 十进制字面量 98_222，使用下划线按三位数字一组隔开
    2. 十六进制字面量 0xff，使用0x开头
    3. 8进制字面量 0o77，使用0o(小写字母o)开头
    4. 二进制字面量 b1111-0000，使用0b开头，按4位数字一组隔开
    5. 字符的字节表示 b'A'，对一个ASCII字符，在其前面加b前缀，直接得到此字符的ASCII码值
    */
    // 1. 十进制字面量
    let decimal = 98_222;
    println!("Decimal: {}", decimal);

    // 2. 十六进制字面量
    let hex = 0xff;
    println!("Hexadecimal: {}", hex);

    // 3. 八进制字面量
    let octal = 0o77;
    println!("Octal: {}", octal);

    // 4. 二进制字面量
    let binary = 0b1111_0000;
    println!("Binary: {}", binary);

    // 5. 字符的字节表示
    let byte_char = b'A';
    println!("Byte character: {}", byte_char);

    // 6. 浮点数表示
    let f32 = 10.0f32;
    println!("Float: {}", f32);

    // 7. 布尔数
    let a = true;
    let b: bool = false;
    println!("Bool {}, {}", a, b);

    // 8. 支持 unicode
    let c = 'z';
    let heart_eyed_cat = '😅';
    let t = '中';
    println!("c = {}, unicode = {}, t = {}", c, heart_eyed_cat, t);

    // 9. 字符串
    let hello = String::from("你好");
    // 你可能想把“你”字取出来，但实际上这样是错误的
    // let a= hello[0];
    println!("{} World", hello);

    // 10. 字符串转义
    //将""号进行转义
    let byte_escape = "I'm saying\"Hello\"";
    println!("{}", byte_escape);
    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);
    // Windows下的换行符
    let byte_escape = "I'm saying\r\n 你好";
    println!("{}", byte_escape);
    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ ok";
    println!("{}", byte_escape);
    // 你可以在 Rust 中显式地添加一个 null 字符到字符串的末尾，
    // 以便与 C 语言中的字符串格式保持一致。
    // let byte_escape = "I'm saying hello.\0";
    // 这行代码演示了如何在字符串后面添加一个 null 字符。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);
    //使用x输入等值的ASCII字符(最高7位)
    let byte_escape = "I'm saying hello \x7f";
    println!("{}", byte_escape);
    // 使用\u{}输入等值的Unicode字符(最高24位)
    let byte_escape = "I'm saying hello \u{0065}";
    println!("{}", byte_escape);
    // 字符串字面量前面加r，表示不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    //这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said:"There is no escape!""#;
    println!("{}", quotes);
    // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号
    // 只要能帮助Rust编译器识别就行
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 11. 字节串
    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string:{:?}", bytestring);
    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes:{:?}", escaped);
    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{2llD} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 12. 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr0 = arr[0];
    println!("{}", arr0);

    // 13. 动态数组
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // println!("{:?}", x); 打印对象，能够快速、清晰地查看变量的内容和结构
    // 为了能够以调试格式打印自定义结构体，需要为结构体实现 Debug trait。
    // 使用 #[derive(Debug)] 可以自动为结构体实现该 trait
    println!("{:?}", v);

    // 14. 哈希表
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // 15. 元组
    let x: (i32, f64, u8) = (500, 6.4, 1);
    // 元组使用.运算符访问其元素，下标从0开始，注意语法
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("({:?}, {:?}, {:?})", five_hundred, six_point_four, one);

    // 16. 结构体,把结构体叫作积类型（product type）
    #[derive(Debug)]
    struct User {
        pub active: bool,
        pub username: String,
        pub email: String,
        age: u64,
    }
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        age: 1,
    };
    println!("{:?}", user1);

    // 17. 枚举类型：和类型（sumtype）
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?},{:?}", four, six);

    // 18. if 不需要使用 （）
    let x = 1;
    //在这里，if else 返回了值
    let y = if x == 0 {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        100
    } else {
        101
    };
    println!("y is {}", y);

    // 19. loop 循环
    let mut counter = 0;
    // 这里，接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值回去
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // 20. while 循环
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 21. for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // 左闭右开区间
    for number in 1..4 {
        println!("{number}");
    }
    println!("--");
    // 左闭右闭区间
    for number in 1..=4 {
        println!("{number}");
    }
    println!("--");
    // 反向
    for number in (1..4).rev() {
        println!("{number}");
    }

    // 22. 函数闭包
    let a = 10u32; // 局部变量
    // 闭包可以捕获函数中的局部变量为我所用，而函数不行
    // fn add_v1(x: u32) -> u32 {
    //     x + a
    // }
    // let result1 = add_v1(20); // 调用函数，报错，
    let add_v2 = |x: u32| -> u32 { x + a }; // 定义一个闭包
    let add_v3 = |x|    { x + a };
    let add_v4 = |x|  x + a;
    
    let result2 = add_v2(20); // 调用闭包
    let result3 = add_v3(20);
    let result4 = add_v4(20);
    println!("result is {result2}, {result3}, {result4}");

    // 23. 模块
    // 与子目录同名的.rs文件，表示这个模块的入口
}
