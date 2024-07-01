
///
/// 定义了一个形状（Shape）枚举，
/// 它有三个变体：
///     长方形 Rectangle
///     三角形 Triangle 
///     圆形 Circle
/// 
#[derive(Debug)]
enum Shape {
    Rectangle,
    Triangle,
    Circle,
}

///
/// enum 中的变体可以挂载各种形式的类型
/// 
enum Shape2 {
    Rectangle { width: u32, height: u32},
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// 给枚举变体一个起始数字值
enum Number {
    Zero = 0,
    One,
    Two,
}
// 给枚举每个变体赋予不同的值
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 都表示一个类型。但是它不能被实例化
enum Foo {}

///
/// 枚举同样能够被 impl
/// 
enum MyEnum {
    Add,
    Subtract,
}
impl MyEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
// impl MyEnum::Subtract { // 错误的(不能对枚举的变体直接 impl)

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool
}

    

fn main() {
    ///
    /// 结构体的实例化需要所有字段一起起作用，
    /// 而枚举的实例化只需要且只能是其中一个变体起作用。
    /// 
    let sharp = Shape::Rectangle;
    let e = WebEvent::Click { x: 320, y: 240 };
    // 
    let shape_a = Shape::Rectangle; // 创建实例
    match shape_a { // 匹配实例
        Shape::Rectangle => {
            println!("{:?}", Shape::Rectangle); // 进了这个分支
        }
        Shape::Triangle => {
            println!("{:?}", Shape::Triangle);
        }
        Shape::Circle => {
            println!("{:?}", Shape::Circle);
        }
    }

    println!("sharp = {:?}", sharp);
    println!("e = {:?}", e);
    //  {:06x} 是格式化参数，这里表示打印出值的 16 进制形式
    println!("roses are #{:06x}", Color::Red as i32);

    // 实例化后执行枚举的方法
    MyEnum::Add.run(100, 200);

    // match 可以有返回值
    // match 表达式里所有的分支都必须处理
    // 是不想处理一些分支，可以用 _ 偷懒
    let shape_a = Shape::Rectangle; // 创建实例
    let ret = match shape_a { // 匹配实例，并返回结果给ret
        Shape::Rectangle => {
            1
        }
        Shape::Triangle => {
            2
        }
        Shape::Circle => {
            3
        }
    };
    println!("{}", ret);

    // 分支分派
    let number = 13;
    // 你可以试着修改上面的数字值，看看下面走哪个分支
    println!("Tell me about {}", number);
    match number {
        // 匹配单个数字
        1 => println!("One!"),
        // 匹配几个数字
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个范围，左闭右闭区间
        13..=19 => println!("A teen"),
        // 处理剩下的情况
        _ => println!("Ain't special"),
    }

    // match 的简化 if let
    let shape_a = Shape::Rectangle;
    if let Shape::Rectangle = shape_a {
        println!("1");
    } else {
        println!("10");
    }

    // while let
    let mut shape_a = Shape::Rectangle;
    let mut i = 0;
    while let Shape::Rectangle = shape_a { // 注意这一句
        if i > 9 {
            println!("Greater than 9, quit!");
            shape_a = Shape::Circle;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            i += 1;
        }
    }

    ///
    /// let 模式匹配的能力
    /// 
    // 创建实例
    let shape_a = Shape2::Rectangle {width: 10, height: 20};
    // 模式匹配出负载内容
    let Shape2::Rectangle {width, height} = shape_a else {
        panic!("Can't extract rectangle.");
    };
    println!("width: {}, height: {}", width, height);

    ///
    /// 元组的析构
    /// 
    let a = (1,2,'a');
    let (b,c,d) = a;
    println!("{:?}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    ///
    /// ref 这个关键字修饰符告诉 Rust 编译器，
    /// 我现在只需要获得那个字段的引用，不要给我所有权
    /// 使用了 ref 后，新定义的 name 变量的值其实是 &a.name ，而不是 a.name
    /// 
    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    let User {
        ref name, // 这里加了一个ref
        age,
        student,
    } = a;
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    println!("{:?}", a);

    ///
    /// 函数参数中的模式匹配
    /// 
    let a = (1,2, 'a');
    foo(a);

    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    bar(a);

}

fn foo((a, b, c): (u32, u32, char)) { // 注意这里的定义
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn bar(User { // 注意这里的定义
    name,
    age,
    student
}: User) {
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}