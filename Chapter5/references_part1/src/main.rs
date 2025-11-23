fn main() {
    // assiging_references();
    // reference_to_reference();
    // comparing_references();
    // borrowing_references_to_arbitrary_expressions();
    references_to_Slices_and_Trait_Objects();
}

fn references_to_Slices_and_Trait_Objects() {
    //references to slices
    let array = [10, 20, 30, 40, 50];
    let slice: &[i32] = &array[1..4]; // a slice referencing elements 20,30,40
    assert_eq!(slice, &[20, 30, 40]);
    assert_eq!(array, &[10, 20, 30, 40, 50]);


    //references to trait objects
    trait Animal {
        fn speak(&self) -> &'static str;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) -> &'static str {
            "Woof!"
        }
    }

    impl Animal for Cat {
        fn speak(&self) -> &'static str {//&'static str is “a reference to a string that lives for the whole program”.
            "Meow!"
        }
    }

    let animals: Vec<&dyn Animal> = vec![&Dog, &Cat];
    for animal in animals {
        println!("{}", animal.speak());
    }
}

fn borrowing_references_to_arbitrary_expressions() {
    //assignment1 : assign &factorial(6) to r, so &factorial(6)'s lifetime is extended to match r's lifetime.

    let r = &factorial(6);
    // Arithmetic operators can see through one level of references. assert_eq!(r + &1009, 1729);
    //r is a reference to the result of factorial(6), which is 720.
    //&1009 is a reference to the integer 1009.

    //assginment2: &1009's lifetime is only for this statement.
    assert_eq!(r + &1009, 1729);
}
fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn comparing_references() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    //because the == operator follows all the references and performs
    //the comparison on their final targets,
    assert!(rrx < rry); //assert fails if we use rx < ry
    assert!(rrx == rry);

    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    // assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
    assert!(rx == *rrx); // this is okay
}

fn reference_to_reference() {
    let x = 10;
    let r = &x;
    let rr = &r;

    //To get the value of x, we need to dereference twice:
    assert_eq!(**rr, 10);

    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    //why not ***rrr?  Because Rust has "auto-dereferencing" for field access.
    //When you use the dot operator to access a field, Rust automatically dereferences as many
    //levels of references as necessary to get to the actual value.
    assert_eq!((***rrr).y, 729);
    assert_eq!(rrr.y, 729);
}

fn assiging_references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = false;
    if b {
        r = &y;
    }

    //r is a reference to either x or y, so you need to dereference it to get the value.
    assert!(*r == 10 || *r == 20);
}
