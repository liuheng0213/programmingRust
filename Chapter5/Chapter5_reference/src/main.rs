fn main() {
    operatordereference();
    // let s = dangling_reference1();
    // dangling_reference3();
    // println!("{}", s);

    // failedTemporaryValues();
    // borrowing_a_Local_Variable();
}

fn lastDemo() {
    let mut x = 42;
    let p = &x;
    assert_eq!(*p, 42);
    //error: cannot assign to x because it is borrowed
    x += 1;
    // if you take out the assignment, this is true
    assert_eq!(*p, 42);
}

// For example, if you build a vector of references,
// all of them must have lifetimes enclosing that of the variable that owns the vector.
fn dangling_reference3() {
    let r;
    {
        let x = 10;
        let v = vec![&x];
        r = v[0];
    }
    // x is gone herem,r is a dangling reference ,
    // pointing to nothing which is not allowed in Rust----invalid memory
    // println!("{:?}", r);
}

//Rust’s complaint is that x lives only until the end of the inner block,
//whereas the reference remains alive until the end of the outer block,
//making it a dangling pointer,
//which is verboten.
fn borrowing_a_Local_Variable() {
    let r;
    {
        let x = 1;
        r = &x;
    }

    // assert_eq!(*r, 1);
}
fn failedTemporaryValues() {
    let r = &factorial(6); // OK — Rust extends the temporary's lifetime
    let x = &&1009; // also OK — but only lives for the statement
    assert_eq!(r, &720);
    assert_eq!(x, &&1009);
    // But this fails:
    let r = &1009; // ❌ temporary does not live long enough
    assert_eq!(r, &1009);
}

fn operatordereference() {
    let r = &factorial(6);
    // Arithmetic operators can see through one level of references.
    //&1009 is a reference to an integer.,+ will dreference it once.
    //assert_eq!(r + &1009, &1729) is not valid because r is a reference to an integer, and &1009 is also a reference to an integer.
    assert_eq!(r + &1009, 1729)
}
fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn dangling_reference1() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    // ❌ ERROR: x does not live long enough
    // println!("{}", r);
}

//The String is dropped when the function ends,
// so the reference can’t live beyond that.
// so returning a reference to it would be a dangling reference.because it cannot be used outside the function scope.
// fn dangling_reference2() -> &String {
//     let s = String::from("hello");
//     &s
// }
