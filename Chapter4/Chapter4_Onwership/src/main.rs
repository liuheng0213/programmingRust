fn main() {
    // print_padovan();
    // box_demo();
    // struct_demo();
    // borrowing_rules();
    // ownership_with_structs();
    // together_example();
    move_vs_copy2();
}

fn drop_or_nor() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here
}

fn move_vs_copy2() {
    // i32 is Copy
    let x = 5;
    let y = x; // x is copied, both x and y are valid
    let z = x; //i32 can be copied multiple times
    println!("x = {}, y = {}", x, y); // OK

    // String is NOT Copy
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2, this is not copy

    // println!("{}", s1); // ❌ ERROR: value borrowed here after move
    println!("{}", s2); // OK

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    //let u = s;//diffrent from i32, vec! is move-only, cannot be copied multiple times
}
/**
 When con‐ trol leaves the scope in which composers is declared,
 the program drops its value and takes the entire arrangement with it.
 Rust programs don’t usually explicitly drop values at all
—they just let values go out of scope.
 */

fn struct_demo() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn box_demo() {
    let point = Box::new((0.625, 0.5)); // point allocated here
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)");
}

//Basic types (like i32, bool, f64) are Copy.
// Others like String are move-only.
fn move_vs_copy1() {
    // i32 is Copy
    let x = 5;
    let y = x; // x is copied, both x and y are valid

    println!("x = {}, y = {}", x, y); // OK

    // String is NOT Copy
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2

    // println!("{}", s1); // ❌ ERROR: value borrowed here after move
    println!("{}", s2); // OK
}

//If a function takes a parameter by value (not &), it takes ownership.
//If you want to use the value again, you either:
// return it back, or
// pass by reference (&String).
fn functions_take_ownership() {
    let s = String::from("hello");

    takes_ownership(s); // ownership of s moves into the function

    // println!("{}", s); // ❌ ERROR: s was moved
}

fn takes_ownership(something: String) {
    println!("In function: {}", something);
    // when function ends, `something` is dropped here
}

//nstead of moving, you can borrow data with &T.
//The function can read it but not take ownership.
fn borrowing() {
    let s = String::from("hello");

    let len = calculate_length(&s); // pass reference, not ownership

    println!("The length of '{}' is {}.", s, len); // s is still valid
}

fn calculate_length(s: &String) -> usize {
    // s is a reference; we don't own it
    s.len()
    // s dropped here? only the reference is dropped, not the String itself
}

//To let a function modify a value without taking ownership, use &mut T.
//Rules:

//The original variable must be mut (let mut s).
//The reference must be &mut s.
//While a mutable reference exists, you can’t use the original variable directly.
fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s); // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}

//Rust enforces:

//Many &T at the same time is OK.
//Only one &mut T at a time.
//You can’t mix &T and &mut T overlapping.
//This prevents data races and keeps memory safe.
fn borrowing_rules() {
    let mut s = String::from("hello");

    // Many immutable borrows: OK
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    println!("r1 = {}, r2 = {}", r1, r2);
    // Immutable borrows end here (last use of r1, r2)

    // One mutable borrow: OK
    let r3 = &mut s;
    r3.push_str("!!!");
    println!("r3 = {}", r3);

    // println!("{}", s); // after r3 is no longer used, s is usable again
}

//Ownership applies to struct fields too.
struct User {
    name: String,
    age: u32,
}

fn ownership_with_structs() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };

    // Move the whole struct
    let user2 = user1;
    // println!("{}", user1.name); // ❌ user1 moved

    println!("{}", user2.name);

    // If we want to just borrow:
    print_user(&user2); // borrow, don't move
}

fn print_user(user: &User) {
    println!("User: {}, age {}", user.name, user.age);
}

fn ownership_box() {
    let b1 = Box::new(10); // heap-allocated i32
    let b2 = b1; // move ownership

    // println!("{}", b1); // ❌ moved
    println!("{}", b2); // OK

    // borrow instead of move:
    let b3 = Box::new((0.625, 0.5));
    print_boxed_point(&b3); // &Box<(f64, f64)> or even & (derefs automatically)
    println!("Still own b3: {:?}", b3);
}

fn print_boxed_point(p: &Box<(f64, f64)>) {
    println!("Point is {:?}", p);
}

//This one shows moves, borrows, and returns all together:
fn together_example() {
    let s: String = String::from("hello");

    // 1. Borrow immutably
    print_once(&s);

    // 2. Borrow mutably
    let mut s = s; // shadow as mutable
    //s in line 193 has been dropped
    add_world(&mut s);

    // 3. Move into a function and get back
    //s in line 199 has been dropped
    let s = take_and_return(s);

    println!("Finally in main: {}", s);
}

fn print_once(s: &String) {
    println!("print_once: {}", s);
}

fn add_world(s: &mut String) {
    s.push_str(", world");
    println!("add_world: {}", s);
}

fn take_and_return(s: String) -> String {
    println!("take_and_return: {}", s);
    s
}
