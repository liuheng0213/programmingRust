fn main() {
    // testSmallest();
    // distinct_Lifetime_Parameters_error();
    // let longer = self_parameter_lifetime("hello22", "worldssss");
    // println!("Longer string: {}", longer);

    // let h = Holder {
    //     name: "Hello".to_string(),
    //     other: "World".to_string(),
    // };

    // let s1 = h.pick_name_part(true);
    // let s2 = h.pick_name_part(false);

    // println!("{s1}, {s2}");

    // sharing_versus_mutation_correct();

    // sharing_versus_mutation_error2();
    // simplest_possible_examples1();
    // simplest_possible_examples2();
    sharedreference()
}

fn sharedreference() {
    let mut x = 42;
    let p = &x;//// shared reference to i32
    assert_eq!(*p, 42);
    x += 1;// // error: cannot assign to x because it is borrowed and p is still in use in line 31
    // if you take out the assignment, this is true
    // assert_eq!(*p, 42);


}

fn simplest_possible_examples2() {
    //It is OK to reborrow a shared reference from a shared reference:
    let mut w = (107, 109);
    let r = &w;
    // ok: reborrowing shared as shared
    let r0 = &r.0;
    // error: can't reborrow shared as mutable
    // let m1 = &mut r.1;
    // r0 gets used here
    println!("{}", r0);

    //=====================================

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; // ok: reborrowing mutable from mutable
    *m0 = 137; //After this line, m0 is never used again
    // ok: reborrowing shared from mutable,
    // and doesn't overlap with m0
    let r1 = &m.0;
    let r2 = &m.1;
    // error: access through other paths still forbidden
    //v.1;

    //if I comment out the below line, r2 ,r1 is not used, so its lifetime ends earlier on line 44
    //then v,1; is ok
    println!("{},{}", r1, r2); // 137,139

    // or move v.1; after r1,r2 are no longer used
    v.1;

    // error: access through other paths still forbidden
    // r1 gets used here
}

fn simplest_possible_examples1() {
    let mut x = 10;
    let r1 = &x; // immutable borrow
    let r2 = &x; // immutable borrow as well ok: multiple shared immutable borrows permitted
    // error: cannot assign to `x` because it is borrowed to r1 r2 as immutable
    //and ri r2 are still in use
    // x += 10;

    // error: cannot borrow `x` as mutable because it is
    // also borrowed as immutable
    //❌ You cannot create a &mut if ANY & references still exist.
    // Since r1 and r2 are still alive, this line also fails.
    // let m = &mut x;

    // the references are used here,
    // so their lifetimes must last
    // at least this long
    // println!("{}, {}, {}", r1, r2, m);
    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y; // error: cannot borrow as mutable more than once
    let z = y; // error: cannot use `y` because it was mutably borrowed println!("{}, {}, {}", m1, m2, z); // references are used here

    //if I comment out the below line, m2 is not used, so its lifetime ends earlier on line 43
    //println!("{}, {}, {}", m1, m2, z); // references are used here
}

fn sharing_versus_mutation_error2() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head); // extend wave with another vector
    extend(&mut wave, &tail); // extend wave with an array

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    //First argument: &mut wave → mutable borrow of wave.
    // Second argument: &wave → immutable borrow of wave.
    //You cannot have a mutable borrow and any other borrow (mutable or immutable) of the same value active at the same time.

    //extend(&mut wave, &wave);

    //correct way to do it:
    let snapshot = wave.clone();
    extend(&mut wave, &snapshot);

    //The extend function’s vec argument borrows wave ...,
    //which has allocated itself a new buffer with space for eight elements.
    //But slice continues to point to the old four-element buffer, which has been dropped.

    //similar to java ConcurrentModificationException
    //Java Hashtable: if you structurally modify the hashtable while iterating,
    //the iterator throws ConcurrentModificationException to stop you from accidentally corrupting things.
    // why clone is needed here:
    //But capacity was 4, so push must:
    // allocate a new buffer (say capacity 8)
    // copy the 4 existing elements into the new buffer
    // free the old 4-element buffer

    // Next iteration of for elt in slice:
    // The iterator is still walking over the old buffer.
    // That memory is already freed.
    // Reading elt now means reading freed memory → undefined behavior.

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}
// fn sharing_versus_mutation_error() {
//     let v = vec![4, 8, 19, 27, 34, 10];
//     let r = &v; // borrow vector `v`
//     let aside = v; // move vector to aside cannot happen because v is borrowed
//     r[0]; // bad: uses `v`, which is now uninitialized
// }

fn sharing_versus_mutation_correct() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0]; // ok: vector is still there }
    } //r goes out of scope here
    // r is no longer used, so we can now move v
    let aside = v;
    print!("haha. {:?}", aside);
}
struct Holder {
    name: String,
    other: String,
}

impl Holder {
    // Return one of two &str slices from inside self
    //lifetime doesnot work for a non-reference type like String
    fn pick_name_part<'a>(&'a self, flag: bool) -> &'a str {
        if flag { &self.name } else { &self.other }
    }
}

fn self_parameter_lifetime<'a>(a: &'a str, b: &'a str) -> &'a str {
    // OK because we explicitly told Rust that both have the same lifetime 'a
    if a.len() > b.len() { a } else { b }
}

//This does not compile. The key issues:

// The return type is &str without a lifetime.
// There are two different input lifetimes: 'a and 'b.
// Lifetime elision rules say:
// If there are multiple input lifetimes and no &self, Rust cannot guess which one to use for the return value.
// So the compiler will complain with something like:
// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from a or b
// fn missing_specific_lifetime_error2<'a, 'b>(a: &'a str, b: &'b str) -> &str {
//     // ❌ ERROR: how long should the return value live?
//     // 'a or 'b? Rust cannot decide.

//     //you’re sometimes returning a: &'a str and sometimes b: &'b str,
//     //and you haven’t told Rust how the return value’s lifetime relates to 'a and 'b.
//     if a.len() > b.len() { a } else { b }
// }

struct S<'a> {
    x: &'a i32,
    y: &'a i32,
}

//Key points:
// S<'a> says both x and y inside S have the same lifetime 'a.
// In let s = S { x: &x, y: &y };:
// &x has the lifetime of x (outer scope, long).
// &y has the lifetime of y (inner scope, shorter).
// Rust must pick one 'a that works for both references.
// It picks the shorter one: the lifetime of y.
// So s.x and s.y both are &'a i32 where 'a = lifetime of y.
//r = s.x;   // r: &'a i32, tied to y’s lifetime
//But you later use r after y’s scope ends:
fn distinct_Lifetime_Parameters_error() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            // s.x and s.y both have lifetime tied to y (the shorter one)
            // because S<'a> requires both to share the same 'a
            let s = S { x: &x, y: &y };
            // s.x has lifetime tied to `y` (inner scope)
            //所以s.x 的lifetime都被缩短到y的生命周期，也就是r的生命周期也是y的生命周期
            r = s.x;
        }
    }
    //So r outlives 'a (the lifetime of y) → would be a dangling reference → Rust rejects it.
    //println!("{}", r); //r is dropped here but it references y which is also dropped
}

struct S1<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

// S<'a, 'b> allows x and y to have different lifetimes.
// In S { x: &x, y: &y }:
// 'a = lifetime of x (outer scope, longer)
// 'b = lifetime of y (inner scope, shorter)
// r = s.x; means r borrows only x, whose lifetime covers the whole function.
// When y and s go out of scope, r is still valid because it points to x, which is still alive.
fn distinct_lifetime_parameters_correct() {
    let x = 10; // lives for the whole function
    let r; // we'll bind r later

    {
        let y = 20; // inner variable with shorter lifetime

        {
            let s = S1 { x: &x, y: &y };
            // s.x has lifetime tied to `x` (outer scope)
            // s.y has lifetime tied to `y` (inner scope)

            r = s.x; // r now borrows `x`, *not* `y`
        } // s is dropped here, but r still points to x (which is still alive)
    } // y is dropped here; s.y is gone, but we never kept a reference to y

    println!("{}", r); // OK: x (and thus r) is still valid here
}

static mut STASH: &i32 = &128;
// fn f(p: &i32) {
//     // still not good enough
//     unsafe {
//         STASH = p;
//     }
// }

fn f(p: &'static i32) {
    // still not good enough
    unsafe {
        STASH = p;
    }
}

// From smallest’s signature,
// we can see that its argument and return value must have the same lifetime, 'a.
// In our call, the argument &parabola must not outlive parabola itself,
// yet smallest’s return value must live at least as long as s.
fn testSmallest() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
    }
    // assert_eq!(*s, 0); should be moved inside the inner scope
    // assert_eq!(*s, 0); // bad: points to element of dropped array
}
// v should have at least one element.
//When a function takes a single reference as an argument
//and returns a single reference,
//Rust assumes that the two must have the same lifetime.
//Writing this out explicitly would give us:
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn testStructLifetime_error() {
    // This does not compile.
    //s.r is a reference that must not outlive the owner of the data.

    // But x dies early → dangling reference → compiler error.
    // struct S {
    //     r: &i32,
    // }
    // let s;
    // {
    //     let x = 10;
    //     s = S { r: &x };
    // }
    // assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}

fn testStructLifetime_correct1() {
    //You do NOT need a lifetime when the struct contains an owned value (T).
    // Because owned data lives inside the struct itself, and does not depend on anything external.
    struct S {
        r: i32, // OWN the integer, not borrow a reference
    }

    let s;
    {
        let x = 10;
        s = S { r: x }; // copy x into the struct
    }
    println!("{}", s.r); // OK — s owns its data
}

fn testStructLifetime_correct2() {
    //     S<'a> means:
    // “This struct contains a reference that must live at least 'a long.”
    // Here 'a becomes the lifetime of x.
    //S contains a reference r that must be valid for the entire lifetime 'a.
    // Therefore, any S<'a> value cannot outlive the data referenced by r.
    //**If r has lifetime 'a, then S<'a> also has lifetime 'a.
    //S cannot outlive the thing that r points to.**
    //You need a lifetime 'a ONLY when the struct contains a reference (&T).
    // Because references borrow data that is owned somewhere else.
    // r's ownership is outside the struct. So we must ensure that r lives long enough.
    //but Rust must ensure lifetime(r) ⊇ lifetime(S<'a>)
    struct S<'a> {
        r: &'a i32,
    }

    let s;
    let x = 10; // ❗ Move `x` OUTSIDE the inner block
    {
        //  let x = 10; // ❗ Move `x` OUTSIDE the inner block
        s = S { r: &x }; //  borrow x into s.r
    }
    assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}
