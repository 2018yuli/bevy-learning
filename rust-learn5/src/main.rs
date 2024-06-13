
// 命名结构体
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Class {
    serial_number: u32,
    grade_number: u32,
    entry_year: String,
    members: Vec<User>,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct ArticleModule;

#[derive(Debug)]
struct RefUser<'a> {
    active: &'a bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: &'a u32,
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为 Rectangle 实现方法
impl Rectangle {
    // area就是方法，被放在impl实现体中
    // self == this 指针
    fn area(self: Self) -> u32 { 
        self.width * self.height
    }
    fn area2(&self) -> u32 {
        self.width * self.height
    }
    fn area3(&mut self) -> u32 {
        self.width * self.height
    }    
}

// impl 可以使用多次
impl Rectangle {
    // 惯用 == 构造函数
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height,
        }
    }
    // 关联函数 == 静态方法
    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }
}


fn main() {
    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 结构体
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    let user0 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user0 {:?}", user0);


    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let mut user1 = User {
        active, // 这里本来应该是 active: active,
        username, // 这里本来应该是 username: username,
        email, // 这里本来应该是 email: email,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("user1 {:?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 注意这里，直接用 ..user1
    };
    println!("user2 {:?}", user2);

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 元组（结构体）
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black {:?}, origin {:?}", black, origin);

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 单元结构体
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    // 连花括号都没有
    let module = ArticleModule;
    println!("module {:?}", module);

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 结构体中的所有权问题
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */
    // 在这里发生了partially moved. 移动后，
    // email 变量拥有了那个值的所有权。而 user1 中的 email 字段就被标记无法访问了
    let email = user1.email;
    // 这一句无法通过编译 (borrow of partially moved value: `user1`)
    // println!("{:?}", user1)
    // borrowed by ..user1 // 注意这里，直接用 ..user1
    // println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    /*
    ////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// 结构体中的所有权问题 - 引用类型
    /// 这种 #[derive(Debug)] 语法在 Rust 中叫属性标注，具体来说这里用的是派生宏属性，
    /// 派生宏作用在下面紧接着的结构体类型上，可以为结构体自动添加一些功能。
    /// 
    ////////////////////////////////////////////////////////////////////////////////////
    */

}
