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
