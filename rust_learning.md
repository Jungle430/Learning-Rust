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

- 单元结构体（不关心数据 `=>`“因为没有数据”，只关心方法）

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
  - `enumerate()` `->` 下标索引
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
  - 逻辑或 `->` `X|Y`

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

---

- 特征对象
  - 使用方面：类似于`OOP`语言中返回基类，然后统一调用基类的方法（接口），但是`Rust`没有继承，利用枚举灵活性很低
  - 以`GUI`为例，所有组件都需要实现`draw`的`trait`

```rust
//trait
pub trait Draw {
    fn draw(&self);
}

//组件
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}

//基类(OOP)里面这就是多态了
pub struct Screen {
    pub components: Vec<?>, //<== 这个 “？” 到底怎么写？
}
```

- `Box`指针+`dyn`特征对象
  - `dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`

```rust
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw> 
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
```

- 所以改善前面的数组

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//依次绘制组件
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- 为什么不用`Vec<T> + where T : draw`的组合呢? `->` 这样里面`<T>`的内容（为了数据大小一样，做不到`Box`那样动态堆上分发了）就一样了，达不到`Box<dyn draw>`的效果

- 类比`OOP`多态即可
- 为什么函数参数只能使用`&dyn`和`Box<dyn ..>` ：因为这两者的大小是已知的，在`stack`上面才能进行`push`和`pop`，<u>如果使用`dyn`不会通过编译</u>

---

- 动态分发
  - 运行时（静态分发是编译时）
  - 阻止编译器有选择的内联方法代码，这会相应的禁用一些优化
  - 性能不如静态分发

---

- `Self`和`self`
  - 一个指代当前的实例对象，一个指代特征或者方法类型的别名

---

- 当一个特征的所有方法都有如下属性时，它的对象才是安全的
  - 方法的返回类型不能是 `Self`
  - 方法没有任何泛型参数

- 标准库中的 `Clone` 特征就不符合对象安全的要求

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

- 因为它的其中一个方法，返回了 `Self` 类型，因此它是对象不安全的。

- `String` 类型实现了 `Clone` 特征， `String` 实例上调用 `clone` 方法时会得到一个 `String` 实例。类似的，当调用 `Vec<T>` 实例的 `clone` 方法会得到一个 `Vec<T>` 实例。`clone` 的签名需要知道什么类型会代替 `Self`，因为这是它的返回值

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}
```

```markdown
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 --> src/lib.rs:2:5
  |
2 |     pub components: Vec<Box<dyn Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone`
  cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`
```

## 深入特征

- 关联类型`Style`

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

---

- 默认泛型类型参数

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

- 默认`RHS`和`Self`一个类型（同类型相加）

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { //无需再次指定RHS类型
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//---

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { //指定RHS类型
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

---

- 调用同名方法
  - 优先调用类型上的方法
  - 如果调用`trait`上面的方法，需要使用`trait::method(para)`的形式

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法
}
```

- 完全限定语法

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

---

- 特征约束
  - 某特征需要其他特征支持

```rust
use std::fmt::Display;

trait OutlinePrint: Display { //需要Display trait的约束
    fn outline_print(&self) {
        let output = self.to_string(); //不然无法调用to_string()
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

- `newtype`装箱：绕过孤儿规则

## 集合类型

- 动态数组`Vec`
  - 当 `Vector` 被删除后，它内部存储的所有内容也会随之被删除
  - 如果里面的内容一旦被引用，事情就复杂很多（生命周期）

- 下标访问和`get`
  - 一个是直接返回值，另一个是`Option<&T>`
  - 越界访问下标会直接崩溃，`get`会返回`None`

- 如果使用不可变借用来指向一个`mut Vec`数组进行更新扩容的时候涉及了新的地址变换，编译器不允许这种情况

- 访问使用迭代，不要用下标+`Range`(上面讲过原理)

- 存储不同元素
  - 枚举：编译时，限制多
  - `dyn trait`：运行时，灵活

---

- 哈希表`HashMap`
  - 平均复杂度为 `O(1)` 的查询方法

- 使用方法

```rust
use std::collections::HashMap;

// 创建一个HashMap，用于存储宝石种类和对应的数量
let mut my_gems = HashMap::new(); //HashMap::with_capacity(capacity)提前分配空间

// 将宝石类型和对应的数量写入表中
my_gems.insert("红宝石", 1);
my_gems.insert("蓝宝石", 2);
my_gems.insert("河边捡的误以为是宝石的破石头", 18);
```

- 堆上运行时内存
- 迭代器+`Collect`创建

```rust
fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect(); //需要显式标注类型
    
    println!("{:?}",teams_map)
}
```

- 不`Copy`的类型进入`HashMap`同样涉及所有权转移，放入引用请考虑生命周期问题~

- 获取元素`get -> Option<&T>`  
  - 或者通过`copied()`从`Option<&T> -> Option<T>`

```rust
let score = scores.get(&team_name).copied().unwrap_or(0);
```

- 迭代循环

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

- 更新查询操作

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
```

- 哈希函数
  - `Key`要实现`std::cmp::Eq` 特征，这样才能做`==`和`!=`的比较

- 若要追求安全，尽可能减少冲突，同时防止拒绝服务（`DoS`）攻击，就要使用密码学安全的哈希函数，`HashMap` 就是使用了这样的哈希函数
  - 其他需求可以使用第三方库

> 目前，`HashMap` 使用的哈希函数是 `SipHash`，它的性能不是很高，但是安全性很高。`SipHash` 在中等大小的 `Key` 上，性能相当不错，但是对于小型的 `Key` （例如整数）或者大型 `Key` （例如字符串）来说，性能还是不够好。若你需要极致性能，例如实现算法，可以考虑这个库：[ahash](https://github.com/tkaitchuck/ahash)。

## 生命周期

- 生命周期很可能是`Rust`中最难的部分

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    } //dropped x
    //r point null
    println!("r: {}", r);
}
```

```markdown
error[E0597]: `x` does not live long enough
 --> src\main.rs:5:13
  |
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("{}", r);
  |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `practice_daily` due to previous error
```

- 生命周期

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
} 
```

- b < a，最后造成了引用指向`null`

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}
```

- 引用的生命周期小于等于本体的生命周期就可以避免该情况，避免了空指针的危险，编译通过

---

- 函数中的生命周期

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```markdown
error[E0106]: missing lifetime specifier
 --> src\main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
```

- 在存在多个引用时，编译器有时会无法自动推导生命周期
  - 此时就需要我们手动去标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析
  - 生命周期标注并不会改变任何引用的实际作用域，**标记的生命周期只是为了取悦编译器，让编译器不要难为我们**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- 该函数签名表明对于某些生命周期 `'a`，函数的两个参数都至少跟 `'a` 活得一样久，同时函数的返回引用也至少跟 `'a` 活得一样久。实际上，这意味着返回值的生命周期与参数生命周期中的较小值一致

- 当把具体的引用传给 `longest` 时，那生命周期 `'a` 的大小就是 `x` 和 `y` 的作用域的重合部分，<u>换句话说，`'a` 的大小将等于 `x` 和 `y` 中较小的那个</u>

---

- 结构体中的生命周期
  - 为了保证结构体内部的引用和结构体本身的生命周期联系上，也需要进行标注

```rust
struct ImportantExcerpt<'a> {
    part: &'a str, //结构体 ImportantExcerpt 所引用的字符串 str 必须比该结构体活得更久!!!，这样里面的引用才不会指空
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
```

- 编译不过的例子

```rust
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}",i);
}
```

---

- 生命周期消除

- 在开始之前有几点需要注意：

  - 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期

  - **函数或者方法中，参数的生命周期被称为 `输入生命周期`，返回值的生命周期被称为 `输出生命周期`**

- 三条消除规则
  - 编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期
  - 若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

1. **每一个引用参数都会获得独自的生命周期**

- 例如一个引用参数的函数就有一个生命周期标注: `fn foo(x: &i32) => fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注:`fn foo(x: &i32, y: &i32) => fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推

2. **若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期**，也就是所有返回值的生命周期都等于该输入生命周期

- 例如函数 `fn foo(x: &i32) -> &i32`，`x` 参数的生命周期会被自动赋给返回值 `&i32`，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &i32 => fn foo<'a>(x: &'a i32) -> &'a i32`

3. **若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期**

- 拥有 `&self` 形式的参数，说明该函数是一个 `方法`，该规则让方法的使用便利度大幅提升

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//==>next
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//==>next
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//lifetime `b <= lifetime 'a like a trait
//==>
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//or
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

---

- **静态生命周期**`'static`
  - 存活和程序一样久
  - 之前见过的：`&str`硬编码值`let s: &'static str = "lalala";`

- 但是要考虑如果真的使用`static`，那么悬空指针怎么办？

## 返回值和错误处理

- `panic!`不可恢复错误

  - 当调用执行该宏时，**程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序**

  - <u>确定程序是彻底寄了再用这个</u>，比如磁盘没电了，机房着火了那种情况

- 更详细的栈展开信息(`Windows`)

```shell
$env:RUST_BACKTRACE=1 ; cargo run
```

- 经典情况，`Vec`下标越界访问，C可以惯着你，Rust直接给你崩掉！

- 两种方式：栈展开和栈终止，配置文件里面自己调就行

- **子线程`panic!`对主程序没有影响（所以任务尽量发给子线程）**
- 最终的输出结果是取决于哪个线程 `panic`
  - 对于 `main` 线程，操作系统提供的终止功能 `core::intrinsics::abort()` 会被调用，最终结束当前的 `panic` 进程
  - 如果是其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 `std::thread::join()` 进行收集

---

- `Result<T,E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 类比`go`里面的`err`，但是这个具有强制处理的性质(`err`程序员自己可以忽略)

- 解构`Err`注意模式匹配，各方面情况考虑（子错误）

- `unwrap`和`expect`
  - 它们的作用就是，如果返回成功，就将 `Ok(T)` 中的值取出来，如果失败，就直接 `panic`，真的勇士绝不多 BB，直接崩溃
  - `expect` 跟 `unwrap` 很像，也是遇到错误直接 `panic`, 但是会带上自定义的错误提示信息，相当于重载了错误打印的函数

```rust
fn main() {
    let f = File::open("hello.txt").unwrap();
}

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

- `?`的使用，同时错误可以通过`From`进行转换
  - <u>大类型统一错误，子类型实现`From`即可</u>

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

- `Result + ? = Err => Option + ? = None` 

```rust
fn first(arr: &[i32]) -> Option<&i32> {
   let v = arr.get(0)?;
   Some(v)
}

//==> Linked call
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- `?` 操作符需要一个变量来承载正确的值

  - `let v = xxx()?;`

  - `xxx()?.yyy()?;`

---

- 带返回值的`main`函数

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

- 返回类型对不上
  - 使用另一种形式

```rust
use std::error::Error; //Rust 中抽象层次最高的错误，其它标准库中的错误都实现了该特征，因此我们可以用该特征对象代表一切错误
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

- `try!`宏（新版本已经被`?`取代，少用）

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
```

---

- 可见性

  - 将结构体设置为 `pub`，但它的所有字段依然是私有的

  - 将枚举设置为 `pub`，它的所有字段也将对外可见

---

## 包，命名空间

- `use`模块冲突：前面依次加上父模块的名称，直到可以被区分

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

- 或者使用`as`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

- `self => use xxx::{self, yyy}`

- `*`引入所有项
