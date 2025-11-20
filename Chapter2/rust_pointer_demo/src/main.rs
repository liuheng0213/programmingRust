// main.rs

// 一个用 Box 做递归数据结构的例子
enum List {
    //The size of a pointer is fixed (usually 8 bytes on 64-bit systems).
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("===== 1. 引用（References）示例 =====");
    references_demo();

    println!("\n===== 2. Box<T> 示例 =====");
    boxes_demo();

    println!("\n===== 3. 原始指针（Raw Pointers）示例 =====");
    raw_pointers_demo();
}

fn diff_between_ref_box() {
    //A Box is an owner, so assigning it moves ownership.
    //onwership moved from b1 to b2
    let b1 = Box::new(42);
    let b2 = b1; // ownership moved
    // println!("{}", b1); // ❌ error: b1 no longer valid
    println!("{}", b2);

    //But references can coexist safely:
    let x = 42;
    let r1 = &x;
    let r2 = &x; // many immutable refs are fine
    println!("{} {}", r1, r2);
}

// ======================= 1. 引用（References） =======================
fn references_demo() {
    // ---------- 1.1 不可变引用 &T ----------
    let x = 5; // x 在栈上
    let r = &x; // r 是一个“借用”x 的不可变引用 &i32

    println!("x = {}, r = {}", x, r);
    // 注意：通过 &T 只能“读”，不能“写”

    // ---------- 1.2 传引用给函数（只读借用） ----------
    let s = String::from("hello");
    let len = string_len(&s); // &s 是 &String
    println!("\"{}\" 的长度是 {}", s, len);
    // 函数只借用 s，不夺走所有权，s 在这里仍然可以用

    // ---------- 1.3 可变引用 &mut T ----------
    let mut y = 10;
    add_one(&mut y); // &mut y：可变引用，可以在函数里修改 y
    println!("y 被 add_one 修改后 = {}", y);

    // ---------- 1.4 引用规则示例 ----------
    let mut s2 = String::from("Rust");

    // 可以有多个不可变引用
    let r1 = &s2;
    let r2 = &s2;
    println!("多个不可变引用：r1 = {}, r2 = {}", r1, r2);

    // r1、r2 在这行之后不再使用，所以它们的“借用作用域”结束
    // 现在可以创建一个可变引用
    let r3 = &mut s2;
    r3.push_str(" is awesome!");
    println!("通过可变引用修改后的 s2 = {}", r3);

    // ❌ 下面这样是非法的（不能同时存在 &mut 和 &）
    // let r1 = &s2;
    // let r2 = &mut s2; // 编译错误：同时有不可变和可变引用

    let mut x = 10;

    let r1 = &mut x; // one mutable reference
    //let r2 = &mut x; // ❌ compile error! cannot borrow `x` as mutable more than once

    *r1 += 5;
    println!("x = {}", r1);
    println!("r1 = {}", r1);
}

// 只读引用示例函数
fn string_len(s: &String) -> usize {
    // 通过引用读数据，不改变所有权
    s.len()
}

// 可变引用示例函数
fn add_one(n: &mut i32) {
    *n += 1; // 解引用，修改原来的值
}
fn box_simple_demo() {
    // 1️⃣ Create a normal tuple on the stack
    let t = (12, "eggs");
    println!("t (on stack) = {:?}", t);

    // 2️⃣ Allocate the same tuple on the heap using Box::new
    let b = Box::new(t);

    // 3️⃣ The type of b is Box<(i32, &str)>
    //    - Box is a pointer that lives on the stack
    //    - The tuple data (12, "eggs") is stored on the heap
    println!("b (Box on heap) = {:?}", b);

    // 4️⃣ Dereference the Box to access its inner value
    println!("The tuple inside the Box = {:?}", *b);

    // 5️⃣ When b goes out of scope (at the end of main),
    //    the heap memory is automatically freed.
    //    You don’t need to call free() or delete() manually — Box handles that.
}
// ======================= 2. Box<T> =======================
fn boxes_demo() {
    // ---------- 2.1 最简单的 Box ----------
    // Box::new 会把值分配到堆上，Box 本身（指针）在栈上
    let b = Box::new(42);
    println!("Box 里的值 b = {}", b); // 使用时像普通引用一样 *b，但 println! 会自动解引用

    // ---------- 2.2 手动解引用 Box ----------
    let v = *b; // 把 Box 里的 i32 拷贝出来（i32 是 Copy 类型）
    println!("手动解引用 Box 得到 v = {}", v);
    //what is reference:  Follow a pointer or reference to get the actual value it points to.

    // ---------- 2.3 使用 Box 实现递归类型（链表） ----------
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("用 Box 构造的链表内容：");
    print_list(&list);
}

fn print_list(list: &List) {
    // 递归打印链表
    match list {
        List::Cons(value, next) => {
            print!("{} ", value);
            print_list(next); // next 是 Box<List>，这里 &*next 或自动解引用
        }
        List::Nil => {
            println!();
        }
    }
}

// ======================= 3. 原始指针（Raw Pointers） =======================
//
// 原始指针： *const T（不可变），*mut T（可变）
// 注意：
//   - 不受借用检查器管理，可能是空指针或悬垂指针
//   - 解引用必须放在 unsafe 块里
fn raw_pointers_demo() {
    // ---------- 3.1 从引用创建原始指针 ----------
    let mut x = 100;

    // 从 &x 得到 *const i32 和 *mut i32
    let p1: *const i32 = &x as *const i32; // 不可变原始指针
    let p2: *mut i32 = &mut x as *mut i32; // 可变原始指针

    // 仅仅“创建”原始指针是安全的，不需要 unsafe
    println!("p1 = {:?}, p2 = {:?}", p1, p2); // 打印的是地址

    // ---------- 3.2 在 unsafe 块中解引用原始指针 ----------
    unsafe {
        // 读 p1 指向的值
        println!("通过 p1 读到的值 = {}", *p1);

        // 通过 p2 修改原值
        *p2 += 50;
        println!("通过 p2 修改后 x = {}", x);
    }

    // ---------- 3.3 从 Box<T> 得到原始指针 ----------
    let b = Box::new(999);
    // &*b 的类型是 &i32，再强转为 *const i32
    let pb: *const i32 = &*b as *const i32;

    unsafe {
        println!("从 Box 得到的原始指针 pb 指向的值 = {}", *pb);
    }
    // 注意：Box 负责在离开作用域时释放堆内存；原始指针只是“看一眼”那块内存

    // ---------- 3.4 空指针（null pointer） ----------
    let null_p: *const i32 = std::ptr::null();
    unsafe {
        if null_p.is_null() {
            println!("null_p 是空指针，不能解引用！");
            // ❌ *null_p 会导致未定义行为
        }
    }

    // ---------- 3.5 悬垂指针（dangling pointer）示例（只演示，不解引用） ----------
    let dangling: *const i32;
    {
        let y = 123;
        dangling = &y as *const i32;
        // 这里 dangling 指向 y 是安全的，但只在这个作用域内
    } // y 在这里被销毁，栈空间被释放

    // 此时 dangling 指向一块已经无效的内存 —— 悬垂指针
    // 我们不会去解引用它，只是打印一下地址：
    println!("dangling 指针的地址 = {:?}", dangling);

    // 如果 千万不要这样做：这是未定义行为！
    // 此时*dangling 已经无效了
    // unsafe {
    //     println!("悬垂指针解引用 = {}", *dangling);
    // }

    //     当 y 离开作用域时，它的栈空间被释放，
    // 此时 dangling 仍然保存着一个指向那块内存的地址。

    // 👉 如果你在 unsafe 块中解引用它（*dangling），
    // 就会访问到无效的内存地址。

    // 这就是“悬垂指针 (dangling pointer)”问题。
}
