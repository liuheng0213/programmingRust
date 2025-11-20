fn main() {
    //dereference: Dereference x (get the value it points to)
    println!("Hello, world!");
    let x = 10;
    //The & symbol — “borrow” or “reference to”
    //Think of it as “look at this thing, but don’t own it.”
    let y = &x;
    println!("{}", y); // 10
    println!("test1 end!");

    let x = 42;
    print_num(&x); // pass a reference
    println!("x is usable: {}", x);
    println!("y is still usable: {}", y);

    let x = 10;
    let y = &x;
    println!("* symbol for y {}==={}", y, x); // prints 10

    let x = 5;
    let y = &x; // y: &i32 (reference to x)
    let z = *y; // z: i32 (copy of value stored at y)
    println!(" z is {}, y is {}, z is {}", z, y, x);
    let mut x = 5;

    let y = &mut x; // y is a mutable reference to x
    *y += 1; // dereference and modify the value
    println!("{}", x); // prints 6

    let x = 10;
    let y = &x; // y is a reference to x
    let mut y_copy = *y; // dereference y and make a mutable copy of the value

    y_copy += 1; // increase the copy
    println!("y_copy = {}", y_copy); // prints 11
    println!("y = {}", y); // prints 11
    println!("x = {}", x); // prints 10

    println!("all end!");
}

fn print_num(mut n: &i32) {
    println!("Number is: {}", n);
}

fn duplicateReference() {
    let mut x = 5;

    {
        let y = &mut x; // y: &mut i32
        *y += 1; // x becomes 6
    } // y goes out of scope here ✅

    println!("{}", x); // ok: prints 6

    let z = &mut x; // ok: new mutable borrow after y is gone
    println!("{}", z); // prints 6
}

fn duplicateReference_2() {
    let mut x = 5;

    let y = &mut x;
    *y += 1; // x = 6
    drop(y); // explicitly end the mutable borrow

    println!("{}", x); // ok

    let z = &mut x; // ok
    println!("{}", z);
}

fn dereference() {
    let x = &10; // x is a reference to 10
    let z = *x; // z = value pointed to by x → 10
    println!("{}", z); // prints 10

    //Because only references or pointers can be dereferenced — not normal integers.
    // let x = 10;
    // let z = *x;
}
