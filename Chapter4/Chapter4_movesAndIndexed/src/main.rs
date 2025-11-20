fn main() {
    // modifyVec();
    dynamically_track();
}

fn dynamically_track() {
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    //composers[0] returns a reference (&Person), not the actual Person.

    //When you try to do .name,
    //Rust sees that you’re attempting to move the value out of that reference (since name is an Option<String>, which is not Copy).
    //That’s illegal — you can’t move a non-Copy field out of a borrowed struct
    // (because it would leave that struct half-empty).
    //because that would invalidate the element inside the vector (Person would be missing a field).
    // let first_name = composers[0].name;
    // println!("first_name {}", first_name);

    // The following code uses only the Rust standard library (`std::mem::replace`),
    // which is available by default and does not require any dependency in Cargo.toml.
    //if we replace the This keeps the struct in a valid state at all times, satisfying Rust’s safety rules.
    //This is a safe operation because it doesn’t invalidate any references to the vector.
    let first_name = std::mem::replace(&mut composers[0].name, None);

    //or simply: let first_name = composers[0].name.take(); which does the same thing.
    //This is a safe operation because it doesn’t invalidate any references to the vector.
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

    println!("first_name {:?}", first_name);
}

fn modifyVec() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    assert_eq!(5, v.len()); //101,102,103,104,105
    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    // 3. Swap in another value for the one we're taking out:
    //The type of v[2] (in expression position) is String, but you cannot move it out directly 
    //because Rust doesn’t allow moving out of a Vec element by indexing like that 
    //(it would leave a “hole” in the vector).
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

fn wrongDemo() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // Pull out random elements from the vector.
    //// error: Cannot move out of index of Vec
    /// because v[2] is a String which does not implement the copy trait
    // let third = v[2];
    // let fifth = v[4]; // here too

    //Now third and fifth are &String (immutable references).
    let third = &v[2];
    let fifth = &v[4]; // here too
}
