fn main() {
    // copydemo();
    // copyDifferentTypes();
    // cloneDemo();
    demo_inbook();
}
fn demo_inbook() {
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };
    print(l);

    //Since Label is not Copy,
    //passing it to print moved ownership of the value to the print function,
    // which then dropped it before returning.
    //But this is silly; a Label is nothing but a u32 with pretensions.
    //There’s no reason passing l to print should move the value.
    //now l is no longer valid here,because it has been moved into print function
    // println!("My label number is: {}", l.number);
}

fn copyDifferentTypes() {
    let a = 10;
    takes_integer(a);
    println!("a still valid: {}", a); // ✅ Copy

    let b = String::from("hello");
    takes_string(b);
    // println!("{}", b); // ❌ moved
}
fn takes_integer(n: i32) {
    println!("Inside function: {}", n);
}

fn takes_string(s: String) {
    println!("Inside function: {}", s);
}
fn copydemo() {
    let l = Label { number: 3 };
    print(l);
    // copies instead of moves
    println!("My label number is: {}", l.number); // OK
}


// If all the fields of your struct are themselves Copy(in this case,u32 is Copy),
// then you can make the type Copy as well 
//by placing the attribute #[derive(Copy, Clone)] above the definition, like so:
#[derive(Copy, Clone)]
struct Label {
    //u32 itself has copy, so the struct Label also has copy
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

//As long as all their fields are Copy, the whole type is too.
//, Clone is necessary in derive for this to work
// You must derive both because Copy extends Clone.
// Without Clone, Copy cannot be implemented.
// And for small, simple types like Point, both just mean “copy the bits.”

//basically Clone is sufficient for the final target,
// however primitive variables have no clone,they have only Copy ,but it needs Clone to make Copy effective.
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn copyPointDemo() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // ❌ moves p1 (since not Copy)
    //println!("{:?}", p1); // copy is necessary
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    location: Point, // Copy + part of Clone
    age: u32,
}

fn cloneDemo() {
    let p1 = Person {
        name: String::from("Alice"),
        location: Point { x: 1, y: 2 },
        age: 21,
    };
    let p2 = p1.clone();

    println!("{:?}\n{:?}", p1, p2);
}
