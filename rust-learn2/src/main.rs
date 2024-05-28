fn main() {
    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// Rust 的数字类型默认有 i8, i16, i32, i64
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    // 
    // i8: [xxxxxxxx] 
    // 这是一个 8 位的有符号整数，能够表示的值的范围是 -128 到 127。
    // 在内存中，这些数值以二进制补码的形式表示。
    // 例如，数值 -1 在内存中的二进制表示为 11111111（补码表示）。
    //
    // i16: [xxxxxxxx] [xxxxxxxx] 
    // 这是一个 16 位的有符号整数，其值的范围是 -32768 到 32767。
    // 同样使用二进制补码形式表示。
    // 例如，数值 -1 在内存中的二进制表示为 1111111111111111。
    //
    // i32: [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] 
    // 这是一个 32 位的有符号整数，可以表示 -2,147,483,648 到 2,147,483,647 的值。
    // 使用二进制补码表示。
    // 例如，数值 -1 在内存中的二进制表示为 11111111111111111111111111111111。
    //
    // i64: [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx] [xxxxxxxx]
    // 这是一个 64 位的有符号整数，其范围是 -9,223,372,036,854,775,808 到 9,223,372,036,854,775,807。
    // 它也使用二进制补码形式表示。
    // 例如，数值 -1 在内存中的二进制表示为 1111111111111111111111111111111111111111111111111111111111111111。
    */

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// Rust 变量与可变性
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    // 一个变量，其内容是否可变，被称作这个变量的可变性（mutability）。
    // mut 叫作可变性修饰符（modifier）
    //  - 如果一个特性不太利于程序的健壮性，但是很好用，滥用的成本非常低，那么它一定会被滥用。
    */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 变量所有权问题：对 = 下手，做骚操作
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    // 正常编译
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");
    
    // 这句代码 println!("{s1}"); 编译异常
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    // println!("{s1}"); // value borrowed here after move
    println!("{s2}");
    
    // 应该修改为
    //  String 是非固定尺寸类型。一般来说，对于固定尺寸类型，会默认放在栈上；
    // 而非固定尺寸类型，会默认创建在堆上，成为堆上的一个资源（浅拷贝）
    let s1 = String::from("I am a superman.");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");
    
    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 为了内存安全：Rust 虽然也是把字符串的引用由 s1 拷贝到了 s2，
    ///     但是只保留了最新的 s2 到字符串的指向，同时却把 s1 到字符串的指向给“抹去”了
    ///     一个变量拥有一个资源的所有权，那它就要负责那个资源的回收、释放
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    // 我们发现 let s2 = s1; 语句执行后，s2 可以使用，而 s1 不能再使用了
    // s1 把值（资源）“移动”给了 s2。既然是移动了，那原来的变量就没有那个值了
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    //println!("{s1}");
    println!("{s2}"); 

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 所有权的基础是三条定义：
    ///     1. Rust 中，每一个值都有一个所有者（变量）。
    ///     2. 任何一个时刻，一个值只有一个所有者。
    ///     3. 当所有者所在作用域（scope）结束的时候，其管理的值会被一起释放掉
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    ///  RAII（ResourceAcquisition Is Initialization）
    ///     堆内存资源随着关联的栈上局部变量一起被回收
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    let s1 = String::from("I am a superman.");
    // s1 处于无效状态（invalid）
    let s2 = s1;
    //println!("{s1}");
    println!("{s2}");


    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    ///  所有权与函数调用
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    let s1 = String::from("I am a superman.");
    foo(s1);
    // println!("{s1}");  s1 的所有权已经被移动进函数里面了
    // foo(s1); s1 的所有权已经被移动进函数里面了

    // 应该写做
    let s1 = String::from("I am a superman.");
    let s1 = bar(s1);
    println!("{s1}");


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

fn foo(s: String) {
    println!("{s}");
}

fn bar(s: String) -> String {
    println!("{s}");
    s
}