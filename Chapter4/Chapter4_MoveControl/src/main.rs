//if / else

// match

// loops (for, while, while let)

fn main() {
    //in if Else
    // ifElseMove();
    //ifElseBorrow();
    //ifElseNoAfterUse();
    // matchMove();
    // matchBorrow1();
    // matchBorrow2();

    // loopMove();
    // loopMoveRef();
    // loopMoveMut();
    // whileLet();
    // partialMove();
    moveAndRefresh();
}

fn moveAndRefresh() {
    // let mut x = vec![10, 20, 30];
    // while f() {
    //     g(x); // move from x
    //     x = h(); // give x a fresh value
    // }
    // e(x);
}


fn partialMove() {
    struct Person {
        name: String,
        age: u32,
    }
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };
    //This is destructuring assignment — it unpacks the fields of the struct p into local variables.
    // Person { name, age: _ } is a pattern.
    // It says: “Take a Person, extract its name field into a variable called name, and ignore the age field.”
    // So after this line:
    // name is now a new variable (of type String), and it takes ownership of p.name.
    // age: _ means “ignore this field” — it’s not bound to a variable.
    // The variable p is moved because its field name was moved out.
    // Move only `name` out
    let Person { name, age: _ } = p;

    // ❌ p.name was moved, but age (a Copy type) is still accessible
    // println!("{}", p.name);  // error
    println!("{}", p.age); // OK, age is Copy
}

fn whileLet() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];

    // pop() returns Option<String>, so it moves the String out
    while let Some(s) = v.pop() {
        println!("popped: {}", s);
    }

    // Now v is empty, but still valid
    println!("v len = {}", v.len()); // 0
}

//Because the borrow (&mut v) lasts only for the duration of the loop.
// Inside the loop:
// Each iteration gives you one temporary mutable reference s to one element.
// After that iteration, s goes out of scope.
// The next iteration borrows the next element.
fn loopMoveMut() {
    let mut v = vec![String::from("a"), String::from("b")];

    for s in &mut v {
        s.push_str("!");
    }

    println!("v = {:?}", v); // ["a!", "b!"]
    println!("v len = {}", v.len()); // 2
}

fn loopMoveRef() {
    let v = vec![String::from("a"), String::from("b")];

    // iterate over &String
    for s in &v {
        println!("{}", s);
    }

    // ✅ v is still accessible
    println!("v = {:?}", v);
}

fn loopMove() {
    let v = vec![String::from("a"), String::from("b")];

    for s in v {
        // each String is moved out of v into s
        println!("{}", s);
    }

    // ❌ ERROR: v was moved into the for loop
    // println!("{:?}", v);
}

fn matchBorrow2() {
    let opt = Some(String::from("hello"));

    match opt {
        // <-- match on &opt
        Some(ref s) => {
            // s: &String (borrow)
            println!("got: {}", s);
        }
        None => {
            println!("nothing");
        }
    }

    // ✅ opt is still usable
    println!("still have opt: {:?}", opt);
}

fn matchBorrow1() {
    let opt = Some(String::from("hello"));

    match &opt {
        // <-- match on &opt
        Some(s) => {
            // s: &String (borrow)
            println!("got: {}", s);
        }
        None => {
            println!("nothing");
        }
    }

    // ✅ opt is still usable
    println!("still have opt: {:?}", opt);
}

fn matchMove() {
    let opt = Some(String::from("hello"));

    //When you write match opt { ... },
    // the opt value is moved into the match expression.
    match opt {
        //So ownership of the inner String is moved into s.
        // Now the String that used to be inside opt belongs to s.
        Some(s) => {
            // s is moved out of opt
            println!("got: {}", s);
        }
        None => {
            println!("nothing");
        }
    }

    // ❌ ERROR: opt was moved in the match
    // println!("{:?}", opt);F
}
fn ifElseNoAfterUse() {
    let s = String::from("hello");
    let condition = true;

    if condition {
        take(s);
    } else {
        take(s);
    }
}

fn ifElseBorrow() {
    let s = String::from("hello");
    let condition = true;

    if condition {
        // immutable borrow
        println!("if branch: {}", s);
    } else {
        println!("else branch: {}", s);
    }

    // still OK to use s
    println!("after if: {}", s);
}

// passing s to another function is moving ownership
fn ifElseMove() {
    let s = String::from("hello");

    let condition = true;

    if condition {
        // ❗ ownership of s moves into take()
        take(s);
    } else {
        // This branch would try to use s
        println!("else branch: {}", s);
    }

    // ❌ ERROR: s might have been moved above
    // println!("after if: {}", s);
}

fn take(s: String) {
    println!("take: {}", s);
}
