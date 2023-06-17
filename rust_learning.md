# Learning Rust

## 引用

- 可以有多个不可变借用，但是<b><font color="red">一旦有了一个可变借用(`&mut`)那么在其周期<u>（直到最后一次使用）</u>内，不能再出现其他借用</font></b>，这样避免了数据竞争

- 引用必须是有效的（避免出现悬垂指针）

## 字符串

- `&str`和`String`的区别（一个不可变引用和一个`Drop`的`Struct`）

- `format!`宏格式化`String`

```rust
fn main() {
    let (x, y, z) = (1, true, 'c');
    let s = format!("{}, {}, {}", x, y, z); //String -> "1, true, c"
    dbg!(s);
}
```

- 通过转义字符`\`来控制字符串输出

```rust
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

- 不转义（原生字符串）可以使用`r`等语法进行控制即可

```rust
let raw_str = r"Escapes don't work here: \x3F \u{211D}";
```

- 遍历（迭代）字符串使用`.chars()`（字符形式）和`.bytes()`（字节）方法

- 非英语字符串切片/子串[**utf8_slice**](https://crates.io/crates/utf8_slice)

- 必须要将结构体实例声明为可变的，才能修改其中的字段，`Rust` 不支持将某个结构体某个字段标记为可变

## 结构体

- 更新结构体时仿制`TypeScript`的语法

```rust
#![allow(unused)]
fn main() {
  let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

//简化

#![allow(unused)]
fn main() {
  let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //尾部使用
    };
}
```

- 但是上面的拷贝碰到非`Copy`的元素会导致<b>结构体部分成员的</b>所有权转移，因此拷贝的时候一定要注意安全

---

- 元组结构体（适用于坐标点或者`RGB`）

```rust
#![allow(unused)]
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

---

- 单元结构体（不关心数据$\rightarrow$“因为没有数据”，只关心方法）

```rust
#![allow(unused)]

struct AlwaysEqual;

trait SomeTrait {
    fn method(&self);
}

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {
    fn method(&self) {
        todo!()
    }
}

fn main() {
    let subject = AlwaysEqual;
    subject.method();
}
```

---

- 打印需要`Display`特征，可以自己定义，如果为了调试过程中查看数据，可以用`derive` 派生+`{:?}/{:#?}`实现

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- 虽然有限制但是很方便快捷
- 还有一个简单的输出 debug 信息的方法，那就是使用 [`dbg!` 宏](https://doc.rust-lang.org/std/macro.dbg.html)，它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。<b>除此之外，它最终还会把表达式值的所有权返回！（利用这个把所有权拿回来）</b>

> - `dbg!` 输出到标准错误输出 `stderr`，而 `println!` 输出到标准输出 `stdout`。

## 枚举

- 可以将数据关联到枚举上简化代码

```rust
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds('A');
}
```

- `Option`

```rust
#![allow(unused)]
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }
}
```

- 通过枚举+模式匹配的方式<b>强制程序员思考处理`null`问题</b>

- [Enum std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html)

## 数组

- `array`——硬编码栈数组，越界访问会造成`panic!`
  - 使用语法糖的时候注意类型是否为`Copy`类型
  ```rust
  let array = [String::from("rust is good!"); 8]; //编译错误，String不是Copy类型
  println!("{:#?}", array);
  ```
  - 补救措施`std::array::from_fn`闭包处理
  ```rust
  #![allow(unused)]
  fn main() {
    let array: [String; 8] = std::array::from_fn(|i| String::from("rust is good!"));
  
    println!("{:#?}", array);
  }
  ```
  
  - 切片·引用`&[T]`和切片`[T]`相比，大小固定，`Rust`更需要这样的数据
- `Vec`——堆动态数组

## 控制流

- `if`语法糖

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

- 注意分支返回值相同

---

- `for`使用的时候注意最好使用集合的引用，防止所有权被代码块“吞噬”
  - `enumerate()` $\rightarrow$ 下标索引
  ```rust
  fn main() {
      let a = [4, 3, 2, 1];
      // `.iter()` 方法把 `a` 数组变成一个迭代器
      for (i, v) in a.iter().enumerate() {
          println!("第{}个元素是{}", i + 1, v);
      }
  }
  ```

- 使用索引`Range`访问会因为边界检查导致性能耗损（这个是运行时），同时非连续访问可能产生脏数据
- 使用迭代访问方法访问连续，且边界问题在编译时期就检查了，保证了高性能和安全

---

- **break 可以单独使用，也可以带一个返回值**，有些类似 `return`
- **loop 是一个表达式**，因此可以返回一个值

## 模式匹配

- `match`
  - `Default`行为用`_/other`表示
  - 每一个分支都必须是一个表达式，且所有分支的表达式<b>最终返回值的类型必须相同</b>，表达式的结果值将作为整个 `match` 表达式的返回值
  - 逻辑或 $\rightarrow$ `X|Y`

- 解构：类比`Option`和`Result`，将里面的值取出来（元组以及复合类型都可以）

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { //解构
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

- 语法糖`if let`（只考虑一种满足条件）

```rust
#![allow(unused)]
fn main() {
    let mut v = Some(3);
    if let Some(3) = v {
        println!("three");
    }

    v = None;
    if let None = v {
        println!("v is None");
    }
}
```

---

- `matches!`宏
- 和`filter()`搭配使用

```rust
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
}

//matches!
v.iter().filter(|x| matches!(x, MyEnum::Foo));
```

```rust
#![allow(unused)]
fn main() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}
```

- 变量遮蔽（所有权交付），避免使用同名变量

```rust
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   if let Some(age) = age {
       println!("匹配出来的age是{}",age); //同名，但是作用域不一样，会出现逻辑错误
   }

   println!("在匹配后，age是{:?}",age);
}
```

- `while let`语法糖

```rust
#![allow(unused)]
fn main() {
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

- 注意`pop()`内部实现（`Option<T>`来强制思考处理`Vec is empty`的情况）

```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
        None
    } else {
        unsafe {
            self.len -= 1;
            Some(ptr::read(self.as_ptr().add(self.len())))
        }
    }
}
```

- `match`中数字和字符的序列匹配

```rust
//number
#![allow(unused)]
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

//char
#![allow(unused)]
fn main() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
```

- 可以解构的对象
  - 结构体、枚举、元组、数组和引用

- 嵌套匹配

```rust
#![allow(unused)]
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
```

- `_s`会绑定值，而`_`不会（所有权转移）
- 忽略值`..`

---

- 匹配守卫
  - 后面加上`if`，相当于多了一个`&&`，<b>如果匹配不上会继续查找而不是`break`退出</b>

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

- 解决覆盖问题

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
```

- `Some(n) if n == y => ...`解决了`Some(y)`这种匹配带来的覆盖问题

- 优先级

```rust
(4 | 5 | 6) if y => ...
```

---

- `@`绑定：解构之后将其绑定到一个变量上面，使得处理式可以使用该值

```rust
#![allow(unused)]
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable) //可以使用对应的值
        }
        Message::Hello { id: 10..=12 } => { //不绑定解构之后使用不了该值
            println!("Found an id in another range")
        }
        Message::Hello { id } => { //不和Range比较也可以使用值
            println!("Found some other id: {}", id)
        }
    }
}
```

- 解构后绑定(1.56)

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);


    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
```

- 或条件绑定(1.53)

```rust
fn main() {
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
```

## Method

- `self,&self,&mut self` 完全形式 `self: Self`
  - `Self`实现方法的结构体类型，这个在后面的`trait`里面很常见

- 理解`Self`

```rust
#[derive(Debug)]
struct Z<T> {
    size: Vec<T>,
    length: usize,
}

trait Add {
    type Style;

    fn add(&mut self, style: Self::Style) -> Result<usize, Box<dyn std::error::Error>>;
}

impl<T> Add for Z<T> {
    type Style = T;

    fn add(&mut self, style: Self::Style) -> Result<usize, Box<dyn std::error::Error>> {
        self.size.push(style); //Self::Style => Z<T>::Style
        self.length = self.size.len();
        Ok(self.length)
    }
}
```

- `Rust`调用方法的时候会自动对`object`进行解引用，不用像`C++`那样使用`->`

- 构造函数`Object::new(para1, para2, ...)`

---

- <b>可以为枚举实现方法</b>

```rust
#![allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move"),
            Message::Write(_) => println!("Write"),
            Message::ChangeColor(_, _, _) => println!("ChangeColor"),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## 泛型

- 大致结构

```rust
fn largest<T>(list: &[T]) -> T {}
```

- 限制`Trait`

```rust
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}
```

- 结构体的泛型类比`C++`即可

- 枚举泛型

```rust
//Option
enum Option<T> {
    Some(T),
    None,
}

//Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 方法泛型（参考[Method](##Method)）
  - 若针对某个类型可以单独实现，比如`impl Point<i32> {}`这种

---

- <b><font color='red'>`const`泛型</font></b>
  - 上面的问题 ：如何不通过引用来处理任意长度的数组？

```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
```

- 编译过程——元编程，<u>根据模板生成对应的代码（没有运行时开销），零成本抽象高性能，代价是更长的编译时间</u>

## 特征`Trait`

- 类比其他语言的`interface`
  - 特征定义了**一组可以被共享的行为，只要实现了特征，你就能使用这组行为**

- 比如泛型中保证两个参数能够相加，使用`std::ops::Add`的`Trait`来进行限制

```rust
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

---

- `trait`定义（类比`java`中的`interface`），若在`mod`中，调用方法应该用`use`引入特征

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

//也可以默认实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
//对应可以直接用该方法或者进行@Override（类比继承即可）

//一些通用的方法可以提前定义好，这样Struct真正实现的时候只用实现特定的一小部分
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

---

- `trait`实现（类比`java`中的`implements interface`）

```rust
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

---

- `trait`调用

```rust
fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}
```

---

- 孤儿规则
  - **如果你想要为类型** `A` **实现特征** `T`**，那么** `A` **或者** `T` **至少有一个是在当前作用域中定义的！**
  - 保证代码不会被不小心破坏

---

- 特征约束

```rust
//基本
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

//多重
pub fn notify<T: Summary + Display>(item: &T) {}

//where（推荐，可读性很好）
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

//为结构体实现方法也可以这么干
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> { //特征约束
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

---

- 常用`Display trait -> ToString`

```rust
fn main() {
    let z = X { a: 1 };
    println!("{}", z.to_string());
}

struct X {
    a: i32,
}

impl ToString for X {
    fn to_string(&self) -> String {
        format!("{}", self.a)
    }
}
```

- 返回特征（类比基类），<u>常用于迭代器处理闭包</u>

```rust
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```

---

- [`derive`派生特征](https://course.rs/appendix/derive.html)
  - 简化常用特征实现
