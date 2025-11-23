fn main() {
    // first_incorrect_demo();
    // test_first_incorrect_demo();
    // function_signature();
    demo2();
}

fn demo2() {
    let x = 10;
    //by passing &x, we are creating a reference to x that has a 'static lifetime.
    //This is because x is defined in the main function, which has a 'static lifetime.
    //This fails to compile: the reference &x must not outlive x, but by passing it to f, we
    // constrain it to live at least as long as 'static. There’s no way to satisfy everyone here,
    // so Rust rejects the code.
    f(&x);
}

fn f(p: &'static i32) {
    println!("p points to {}", p);
}

fn function_signature() {
    struct Holder<'a> {
        r: &'a i32,
    }

    static mut H: Option<Holder<'static>> = None;
    //g cannot store p inside a struct that outlives the calling function.
    // fn g<'a>(p: &'a i32) {
    //     unsafe {
    //         // ❌ can't store short-lived ref into static
    //         H = Some(Holder { r: p });
    //     }
    // }
}

fn test_first_incorrect_demo() {
    // To call f, we need to pass it a reference with a 'static lifetime.
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}

fn first_incorrect_demo() {
    // This code has several problems, and doesn't compile.
    // static mut STASH: &i32;
    // fn f(p: &i32) {
    //     unsafe {
    //         STASH = p;
    //     }
    // }

    //corrected version
    //static mut STASH: &i32; is invalid — Rust requires static values to be initialized.
    static mut STASH: &i32 = &10;

    // The parameter p needs to have the 'static lifetime, because we are assigning it to a static variable.
    //Since STASH lives for the program’s entire execution, the reference type it holds must
    //have a lifetime of the same length;
    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
}

fn Receiving_References_as_Function_Arguments() {}
