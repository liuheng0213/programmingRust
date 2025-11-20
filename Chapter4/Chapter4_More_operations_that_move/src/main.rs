fn main() {}

fn demo() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here

    //This time, t has taken ownership of the original string from s,
    //so that by the time we assign to s,
    //it is uninitialized. In this scenario, no string is dropped.
    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here
}
