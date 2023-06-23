# Learning Rust(Advanced learning)

## 生命周期

- 无界生命周期
  - 不安全代码(`unsafe`)经常会凭空产生引用或生命周期，这些生命周期被称为是 **无界(unbound)** 的。

```rust
fn f<'a, T>(x: *const T) -> &'a T {
    unsafe {
        &*x
    }
}
```

- 上述代码中，参数 `x` 是一个裸指针，它并没有任何生命周期，然后通过 `unsafe` 操作后，它被进行了解引用，变成了一个 Rust 的标准引用类型，该类型必须要有生命周期，也就是 `'a`。

- 可以看出 `'a` 是凭空产生的，因此它是无界生命周期。这种生命周期由于没有受到任何约束，因此它想要多大就多大，这实际上比 `'static` 要强大。例如 `&'static &'a T` 是无效类型，但是无界生命周期 `&'unbounded &'a T` 会被视为 `&'a &'a T` 从而通过编译检查，因为它可大可小，就像孙猴子的金箍棒一般。

- 我们在实际应用中，要尽量避免这种无界生命周期。最简单的避免无界生命周期的方式就是在函数声明中运用生命周期消除规则。**若一个输出生命周期被消除了，那么必定因为有一个输入生命周期与之对应**。

---

- 生命周期约束`HRTB`

- `'a: 'b`
  - 假设有两个引用 `&'a i32` 和 `&'b i32`，它们的生命周期分别是 `'a` 和 `'b`，若 `'a` >= `'b`，则可以定义 `'a:'b`，表示 `'a` 至少要活得跟 `'b` 一样久。

```rust
struct DoubleRef<'a,'b:'a, T> {
    r: &'a T,
    s: &'b T
}
```

- `T: 'a`
  - 表示类型 `T` 必须比 `'a` 活得要久：

```rust
struct Ref<'a, T> {
    r: &'a T
}
```

- 因为结构体字段 `r` 引用了 `T`，因此 `r` 的生命周期 `'a` 必须要比 `T` 的生命周期更短(被引用者的生命周期必须要比引用长)。

---

- 闭包函数的消除规则

```rust
fn fn_elision(x: &i32) -> &i32 { x }
let closure_slision = |x: &i32| -> &i32 { x };
```

```markdown
error: lifetime may not live long enough
  --> src/main.rs:39:39
   |
39 |     let closure = |x: &i32| -> &i32 { x }; // fails
   |                       -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                       |        |
   |                       |        let's call the lifetime of this reference `'2`
   |                       let's call the lifetime of this reference `'1`

```

- ???

- 由于闭包涉及了其他程序作用域内变量的生命周期（`move`)，所以不和函数使用一套生命周期消除规则
  - 说白了就是如果玩的不溜别拿闭包装逼了

- 可以使用`Fn trait`解决部分问题

```rust
fn main() {
    let closure_slision = fun(|x: &i32| -> &i32 { x });
    assert_eq!(*closure_slision(&45), 45);
    // Passed !
}

fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}
```

---

- `NLL(Non-Lexical Lifetime)`

- **引用的生命周期从借用处开始，一直持续到最后一次使用的地方**，而不是早期版本直到作用域结束，这样程序简洁了很多

---

- `Reborrow`再借用

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r;

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

- `rr` 是对 `r` 的再借用

- 对于再借用而言，`rr` 再借用时不会破坏借用规则，但是你不能在它的生命周期内再使用原来的借用 `r`，来看看对上段代码的分析

```rust
fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // reborrow! 此时对`r`的再借用不会导致跟上面的借用冲突
    let rr: &Point = &*r;

    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    println!("{:?}", rr);

    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

- 函数体内对参数的二次借用也是典型的 Reborrow 场景

```rust
use std::vec::Vec;
fn read_length(strings: &mut Vec<String>) -> usize {
   strings.len() //==>len(&self)
}
```

---

- 新增生命周期消除规则
  - `impl`块消除

```rust
impl<'a> Reader for BufReader<'a> {
    // methods go here
    // impl内部实际上没有用到'a
}

impl Reader for BufReader<'_> {
    // methods go here
}
//'_ 生命周期表示 BufReader 有一个不使用的生命周期，我们可以忽略它，无需为它创建一个名称
```

- 生命周期约束消除

```rust
// Rust 2015
struct Ref<'a, T: 'a> {
    field: &'a T
}

// Rust 2018
struct Ref<'a, T> {
    field: &'a T
}
```

---

- `&'static`和`T: 'static`

- `&' static`
  - `&str`
  - 特征对象——隐式生命周期

- `&'static` 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 `&'static`
- 对于字符串字面量来说，它<b>直接被打包到二进制文件中，永远不会被 `drop`，因此它能跟程序活得一样久，自然它的生命周期是 `'static`</b>
- <b>`&'static` 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则</b>

- `&'static` 的引用确实可以和程序活得一样久，因为我们通过 `get_str_at_location` 函数直接取到了对应的字符串
- 持有 `&'static` 引用的变量，它的生命周期受到作用域的限制，大家务必不要搞混了

---

- `T: 'static`
- 在以下两种情况下，`T: 'static` 与 `&'static` 有相同的约束：`T` 必须活得和程序一样久

```rust
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let i = 5;

    print_it(&i);
    print_it1(&i);
}
```

```markdown
error[E0597]: `i` does not live long enough
  --> src\main.rs:14:14
   |
14 |     print_it(&i);
   |     ---------^^-
   |     |        |
   |     |        borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
15 |     print_it1(&i);
16 | }
   | - `i` dropped here while still borrowed
```

- 原因很简单: `&i` 的生命周期无法满足 `'static` 的约束，如果将 `i` 修改为常量，那自然一切 OK

```rust
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

const I: i32 = 2;

fn main() {
    print_it(&I);
    print_it1(&I);
}
```

- 另一种修改方法

```rust
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let i = 5;

    print_it(&i);
}
```

- 这段代码竟然不报错了！原因在于我们约束的是 `T`，但是使用的却是它的引用 `&T`，换而言之，我们根本没有直接使用 `T`，因此编译器就没有去检查 `T` 的生命周期约束！它只要确保 `&T` 的生命周期符合规则即可，在上面代码中，它自然是符合的

- 例子

```rust
use std::fmt::Display;

fn main() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // -> 42
    println!("&'static str: {}", r2); // -> &'static str

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        static_bound(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        r3 = &s1;

        // s1 在这里被 drop
    }
    println!("{}", r3);
}

fn static_bound<T: Display + 'static>(t: &T) {
    println!("{}", t);
}
```

- 如果你需要添加 `&'static` 来让代码工作，那很可能是设计上出问题了
- 如果你希望满足和取悦编译器，那就使用 `T: 'static`，很多时候它都能解决问题

- `&'static` 声明变量指向的内存，是具有static lifetime的。但是这个变量本身还会在超出`block`时被释放
- `T：‘static` 声明的是T这个类型的lifetime是被static约束的，当然也就比static长。注意，T可能是&类型

## 闭包`Closure`

- 其他编程语言里面的匿名函数
  - <b>允许捕获调用者作用域中的值</b>

```rust
fn main() {
   let x = 1;
   let sum = |y| x + y;

    assert_eq!(3, sum(2));
}
```

- 形式

```markdown
|param1, param2,...| {
    语句1;
    语句2;
    返回表达式
}

只有一个返回表达式
|param1| 返回表达式
```

- 不同形式

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

- 虽然类型推导很好用，但是它不是泛型，<b>当编译器推导出一种类型后，它就会一直使用该类型</b>

---

- 结构体中的闭包
  - 利用`Fn trait`
  - <u>其实 Fn 特征不仅仅适用于闭包，还适用于函数</u>，因此 `query` 字段除了使用闭包作为值外，还能使用一个具名的函数来作为它的值

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

- 更多泛型

```rust
struct Cacher<T, V>
where
    T: Fn(V) -> V,
    V: Copy,
{
    query: T,
    value: Option<V>,
}

impl<T, V> Cacher<T, V>
where
    T: Fn(V) -> V,
    V: Copy,
{
    fn new(query: T) -> Cacher<T, V> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

- 当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担

- 三种`Fn`
  - `FnOnce`，该类型的闭包<b>会拿走被捕获变量的所有权</b>。`Once` 顾名思义，说明该闭包只能运行一次
    - 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 `move` 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程
  - `FnMut`，它以<b>可变借用</b>的方式捕获了环境中的值，因此可以修改该值
  - ```rust
    fn main() {
        let mut s = String::new();
    
        let update_string = |str| s.push_str(str);
    
        exec(update_string);
    
        println!("{:?}", s);
    }
    
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        //闭包不能使用三条法则自动加入，所以要手动标注
        f("hello")
    }
    ```
  
  
  - `Fn` 特征，它以不可变借用的方式捕获环境中的值

---

- `move`和`Fn`
  - **一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们**。`move` 本身强调的就是后者

```rust
fn main() {
    let s = String::new();

    let update_string = move || println!("{}", s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}
```

- 我们在上面的闭包中使用了 `move` 关键字，所以我们的闭包捕获了它，但是由于闭包对 `s` 的使用仅仅是不可变借用，因此该闭包实际上**还**实现了 `Fn` 特征(该闭包不仅仅实现了 `FnOnce` 特征，还实现了 `Fn` 特征)

```rust
fn main() {
    let s = String::new();

    let update_string = move || println!("{}", s);

    exec(update_string);
}

fn exec<F: Fn()>(f: F) {
    f()
}
```

- 改成`Fn`也可以过编译

---

- 三种`Fn`的关系

  - 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次

  - 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征

  - 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征

- 源码

```rust
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

-  `Fn` 的前提是实现 `FnMut`
- `FnMut` 的前提是实现 `FnOnce`
  - 因此要实现 `Fn` 就要同时实现 `FnMut` 和 `FnOnce`

---

- 闭包作为函数的返回值
  - 由于闭包的大小不确定，所以使用`Box`

```rust
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```

## 迭代器`Iterator`

- 数组实现了 `IntoIterator` 特征，Rust 通过 `for` 语法糖，自动把实现了该特征的数组类型转换为迭代器

- `IntoIterator` 特征拥有一个 `into_iter` 方法，因此我们还可以显式的把数组转换成迭代器

- 迭代器是惰性的，意味着如果你不使用它，那么它将不会发生任何事
  - 创建了不用/或到使用时不会有任何行为，保证了创建的时候不会有任何额外的性能损耗，元素也不会被消耗

- 迭代的特征方法：`next`

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 省略其余有默认实现的方法
}
```

- 手动迭代必须将迭代器声明为 `mut` 可变，因为调用 `next` 会改变迭代器其中的状态数据（当前遍历的位置等），而 `for` 循环去迭代则无需标注 `mut`，因为它会帮我们自动完成

- `next` 方法对**迭代器的遍历是消耗性的**，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 `None`

- 迭代器自身也实现了 `IntoIterator`(自己也可以来一下`into_iter`，当然返回的也是自己)

---

- 转化为迭代器的种类（重要）
  - `into_iter` 会夺走所有权
  - `iter` 是借用
  - `iter_mut` 是可变借用

- <b>`into_` 之类的，都是拿走所有权，`_mut` 之类的都是可变借用，剩下的就是不可变借用</b>

---

-  `collect` 方法：该方法就是一个消费者适配器，使用它可以将一个迭代器中的元素收集到指定类型中（手动标注）

---

- 适配器：刷力扣吧！

---

- 迭代器是 Rust 的 **零成本抽象**（zero-cost abstractions）之一，意味着抽象并不会引入运行时开销，这与 `Bjarne Stroustrup`（C++ 的设计和实现者）在 `Foundations of C++（2012）` 中所定义的 **零开销**（zero-overhead）如出一辙
  - 翻译一下：用就得了！

## `Box<T>`堆对象分配

- 堆栈性能
  - 小型数据，在栈上的分配性能和读取性能都要比堆上高
  - 中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
  - 大型数据，只建议在堆上分配和使用

- 栈的分配速度肯定比堆上快，但是读取速度往往取决于你的数据能不能放入寄存器或 CPU 高速缓存。 因此不要仅仅因为堆上性能不如栈这个印象，就总是优先选择栈，导致代码更复杂的实现

- `Box`使用场景

  - 特意的将数据分配在堆上

  - 数据较大时，又不想在转移所有权时进行数据拷贝

  - 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时

  - 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

- 例子

```rust
fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
}
```

- `println!` 可以正常打印出 `a` 的值，是因为它隐式地调用了 `Deref` 对智能指针 `a` 进行了解引用
- 最后一行代码 `let b = a + 1` 报错，是因为在表达式中，我们无法自动隐式地执行 `Deref` 解引用操作，你需要使用 `*` 操作符 `let b = *a + 1`，来显式的进行解引用
- `a` 持有的智能指针将在作用域结束（`main` 函数结束）时，被释放掉，这是因为 `Box<T>` 实现了 `Drop` 特征

---

- <b>当栈上数据转移所有权时，实际上是把数据拷贝了一份，最终新旧变量各自拥有不同的数据，因此所有权并未转移</b>

- 而堆上则不然，底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，再将新的指针赋予新的变量，然后让拥有旧指针的变量失效，最终完成了所有权的转移
- 大块数据避免在栈上面拷贝

```rust
use std::fmt::Debug;

fn main() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;
    p(arr);
    p(arr1);
    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

fn p<T>(a: T)
where
    T: Copy + Debug,
{
    println!("{:#?}", a)
}
```

---

- `Box`内存布局

- `Vec<i32>`

```
(stack)    (heap)
┌──────┐   ┌───┐
│ vec1 │──→│ 1 │
└──────┘   ├───┤
           │ 2 │
           ├───┤
           │ 3 │
           ├───┤
           │ 4 │
           └───┘
```

- `vec<Box<_>>`

```
                    (heap)
(stack)    (heap)   ┌───┐
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec2 │──→│B1 │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │B2 │───→│ 2 │
           ├───┤    └───┘
           │B3 │─┐  ┌───┐
           ├───┤ └─→│ 3 │
           │B4 │─┐  └───┘
           └───┘ │  ┌───┐
                 └─→│ 4 │
                    └───┘
```

- `Deref`可以部分情况下解引用，但是表达式情况下不能

---

- `Box::leak`
  - `Box` 中还提供了一个非常有用的关联函数：`Box::leak`，它可以消费掉 `Box` 并且强制目标值从内存中泄漏

- 其实还真有点用，例如，你可以把一个 `String` 类型，变成一个 `'static` 生命周期的 `&str` 类型

```rust
fn main() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
```

- 我还可以手动为变量标注 `'static` 啊。其实你标注的 `'static` 只是用来忽悠编译器的，但是超出作用域，一样被释放回收。而使用 `Box::leak` 就可以将一个运行期的值转为 `'static`

- 一个简单的场景，**你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久**，那么就可以使用 `Box::leak`，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 `Rc/Arc` 也可以实现此功能，但是 `Box::leak` 是性能最高的

## `Deref`解引用

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age}
    }

    fn display(self: &mut Person, age: u8) {
        let Person{name, age} = &self;
    }
}
```

- 为啥套了两层`&&`还能正常使用？ `=>` `Deref`解引用
  - 目的：不让你写出来`*****s`形式的解引用

- 智能指针的名称来源，主要就在于它实现了 `Deref` 和 `Drop` 特征，这两个特征可以智能地帮助我们节省使用上的负担：

  - `Deref` 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 `*T`

  - `Drop` 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

---

- 实现 `Deref` 后的智能指针结构体，就可以<b>像普通引用一样</b>，通过 `*` 进行解引用，例如 `Box<T>` 智能指针

```rust
fn main() {
    let x = Box::new(1);
    let sum = *x + 1;  // Box<i32> => i32
}
```

- `Deref`的实现

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//==> Deref
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

- `deref` 返回的是一个常规引用，可以被 `*` 进行解引用

- `*`背后的原理
  - 实际过程

```rust
*(y.deref())
```

- 若一个类型实现了 `Deref` 特征，那<b>它的引用</b>在传给函数或方法时，会根据参数签名来决定是否进行隐式的 `Deref` 转换

```rust
fn main() {
    let s = String::from("hello world");
    display(&s) //注意传引用
}

fn display(s: &str) {
    println!("{}",s);
}
```

- `Deref` 可以支持连续的隐式转换

```rust
fn main() {
    let s = MyBox::new(String::from("hello world")); //deref trait
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}
```

- 靠`IDE`智能提示会好很多
- 其他例子

```rust
fn main() {
    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();
}
```

- 多层会自动解包

---

- 当 `T: Deref<Target=U>`，可以将 `&T` 转换成 `&U`，也就是我们之前看到的例子
- 当 `T: DerefMut<Target=U>`，可以将 `&mut T` 转换成 `&mut U`
- 当 `T: Deref<Target=U>`，可以将 `&mut T` 转换成 `&U`

## `Drop`释放资源

- `Drop`特征：超出作用域编译器自动调用的东西（类似于析构函数）

- `Drop` 特征中的 `drop` 方法借用了目标的可变引用，而不是拿走了所有权，这里先设置一个悬念，后边会讲
- 结构体中每个字段都有自己的 `Drop`

- <b>`Rust` 自动为几乎所有类型都实现了 `Drop` 特征，因此就算你不手动为结构体实现 `Drop`，它依然会调用默认实现的 `drop` 函数，同时再调用每个字段的 `drop` 方法</b>

---

- <b>`drop`虽然用的是`&mut self`，但是`drop`之后对应的值也无法使用了（被拿走了所有权）</b>
  - 同时不允许显式的调用`drop`函数(`foo.drop()`这种)

```rust
#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

fn main() {
    let foo = Foo;
    drop(foo);
    println!("Running!:{:?}", foo);
}

//其实drop签名是这样
pub fn drop<T>(_x: T)
```

```markdown
error[E0382]: borrow of moved value: `foo`
  --> src\main.rs:13:31
   |
11 |     let foo = Foo;
   |         --- move occurs because `foo` has type `Foo`, which does not implement the `Copy` trait
12 |     drop(foo);
   |          --- value moved here
13 |     println!("Running!:{:?}", foo);
   |                               ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

- 对于` Drop `而言，主要有两个功能（类比析构函数）

  - 回收内存资源

  - 执行一些收尾工作

---

- `Copy`和`Drop`互斥
  - 实现了 `Copy` 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。因此这些实现了 `Copy` 的类型无法拥有析构函数

## `Rc`和`Arc`

- 通过引用计数的方式，允许一个数据资源在同一时刻拥有多个所有者（情景：图，多线程）

---

- `Rc<T>`
  - `reference counting`
    - 通过记录一个数据被引用的次数来确定该数据是否正在被使用。<b>当引用次数归零时，就代表该数据不再被使用，因此可以被清理释放</b>

```rust
fn main() {
    let s = String::from("hello, world");
    // s在这里被转移给a
    let a = Box::new(s);
    // 报错！此处继续尝试将 s 转移给 b
    let b = Box::new(s);
}

//Rc
use std::rc::Rc;
fn main() {
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
}
```

- `Rc::clone`

  - 这里的 `clone` **仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据**，因此 `a` 和 `b` 是共享了底层的字符串 `s`，这种**复制效率是非常高**的。当然你也可以使用 `a.clone()` 的方式来克隆，但是从可读性角度，我们更加推荐 `Rc::clone` 的方式

  - 实际上在 Rust 中，还有不少 `clone` 都是浅拷贝，例如[迭代器的克隆](https://course.rs/compiler/pitfalls/iterator-everywhere.html)

- 使用关联函数 `Rc::strong_count` 可以获取当前引用计数的值

```rust
use std::rc::Rc;
fn main() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

- 因为`Drop`，`Rc`离开作用域的时候引用计数会- 1

- 引用计数为0的时候对应的资源也会自动释放掉

---

- 可变借用只能唯一

- `Rc`需要与其他数据类型配合使用保证上述条件

```rust
use std::rc::Rc;

struct Owner {
    name: String,
    // ...其它字段
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...其它字段
}

fn main() {
    // 创建一个基于引用计数的 `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });

    // 创建两个不同的工具，它们属于同一个主人
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // 释放掉第一个 `Rc<Owner>`
    drop(gadget_owner);

    // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
    // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
    // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
    // 引用指向底层的 owner 数据，引用计数尚未清零
    // 因此 owner 数据依然可以被使用
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
    // 数据也被清理释放
}
```

---

- 线程并发专用原子`Rc => Arc`
  - 保证我们的数据能够安全的在线程间共享
  - 会有不算小的性能损耗（和`Rc`做对比），二者也不在一个模块

## `Cell`和`RefCell`

- `Cell` 和 `RefCell` 在功能上没有区别，区别在于 `Cell<T>` 适用于 `T` 实现 `Copy` 的情况：

| Rust 规则                            | 智能指针带来的额外规则                  |
| ------------------------------------ | --------------------------------------- |
| 一个数据只有一个所有者               | `Rc/Arc`让一个数据可以拥有多个所有者    |
| 要么多个不可变借用，要么一个可变借用 | `RefCell`实现编译期可变、不可变引用共存 |
| 违背规则导致**编译错误**             | 违背规则导致**运行时`panic`**           |

- `RefCell` 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，从编译器错误变成了 `panic` 异常

```rust
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}
```

- 违背借用规则，直接`panic!`

---

- 与 `Cell` 用于可 `Copy` 的值不同，`RefCell` 用于引用
- `RefCell` 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
- `RefCell` 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
- 使用 `RefCell` 时，违背借用规则会导致运行期的 `panic`

---

- `Cell` 只适用于 `Copy` 类型，用于提供值，而 `RefCell` 用于提供引用
- `Cell` 不会 `panic`，而 `RefCell` 会
- `Cell` 没有额外的性能损耗

---

- `Rc<T>和RefCell<T>`结合使用可以一个拥有多重所有权的可变数据

## 循环引用

- 循环引用由于图中形成环，导致引用计数无法归零，造成内存泄漏（有单边才有`Drop`的可能，`>=`两个边就`Drop`不出去了）

- `Weak` 非常类似于 `Rc`，但是与 `Rc` 持有所有权不同，`Weak` 不持有所有权，它仅仅保存一份指向数据的弱引用：如果你想要访问数据，需要通过 `Weak` 指针的 `upgrade` 方法实现，该方法返回一个类型为 `Option<Rc<T>>` 的值
  - <font color="red">何为弱引用？就是<b>不保证引用关系依然存在</b>，如果不存在，就返回一个 `None`！</font>
  - 弱引用的计数不影响对象的清理

| `Weak`                                          | `Rc`                                      |
| ----------------------------------------------- | ----------------------------------------- |
| 不计数                                          | 引用计数                                  |
| 不拥有所有权                                    | 拥有值的所有权                            |
| 不阻止值被释放(drop)                            | 所有权计数归零，才能 drop                 |
| 引用的值存在返回 `Some`，不存在返回 `None`      | 引用的值必定存在                          |
| 通过 `upgrade` 取到 `Option<Rc<T>>`，然后再取值 | 通过 `Deref` 自动解引用，取值无需任何操作 |

- 个人理解：在图，树的结构里面肯定有两个边互相指的环（比如树根指叶子，叶子指根），如果说`RcCell<Rc>`是实线，那么`Weak<T>`就是虚线，我们要虚实结合防止实线连成环造成循环引用

---

- 结构体自引用

```rust
fn main() {
    let s = "aaa".to_string();
    let v = SelfRef {
        value: s,
        pointer_to_value: &s,
    };
}

struct SelfRef<'a> {
    value: String,

    // 该引用指向上面的value
    pointer_to_value: &'a str,
}
```

```
error[E0382]: borrow of moved value: `s`
 --> src\main.rs:5:27
  |
2 |     let s = "aaa".to_string();
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let v = SelfRef {
4 |         value: s,
  |                - value moved here
5 |         pointer_to_value: &s,
  |                           ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |         value: s.clone(),
  |                 ++++++++
```

- 使用`Option`解决

```rust
#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

fn main() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);

    println!("{:?}", tricky);
}
```

- 限制较多

---

- `unsafe`实现

```rust
#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer_to_value: *mut String,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        SelfRef {
            value: String::from(txt),
            pointer_to_value: std::ptr::null_mut(),
        }
    }

    fn init(&mut self) {
        let self_ref: *mut String = &mut self.value;
        self.pointer_to_value = self_ref;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_to_value(&self) -> &String {
        assert!(
            !self.pointer_to_value.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.pointer_to_value) }
    }
}

fn main() {
    let mut t = SelfRef::new("hello");
    t.init();
    println!("{}, {:p}", t.value(), t.pointer_to_value());

    t.value.push_str(", world");
    unsafe {
        (&mut *t.pointer_to_value).push_str("!");
    }

    println!("{}, {:p}", t.value(), t.pointer_to_value());
}
```

- 我们在 `pointer_to_value` 中直接存储裸指针，而不是 Rust 的引用，因此不再受到 Rust 借用规则和生命周期的限制，而且实现起来非常清晰、简洁。但是缺点就是，通过指针获取值时需要使用 `unsafe` 代码

---

- `Pin`固定住一个值，防止该值在内存中被移动
