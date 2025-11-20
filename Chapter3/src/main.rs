fn main() {
    println!("Hello, world!");
    print();

    printRawString();

    format();

    string_onwnership_movement();
}
fn string_onwnership_movement() {
    {
        let s = String::from("hello world");
        println!("Inside inner scope: {}", s);
        // `s` is valid here
    }
    // <- s goes out of scope here, and its memory is automatically freed
    // println!("Exited inner scope {}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // ❌ error: borrow of moved value `s1`
    println!("s2: {}", s2);

    //correct copy
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy of the heap buffer

    println!("s1 = {}, s2 = {}", s1, s2);
}
fn format() {
    //Insert the next value (23), also 2-digit padded → 23
    assert_eq!(
        format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string()
    );
}

// To use `use regex::Regex;` you need to add the following to your Cargo.toml:
// [dependencies]
// regex = "1.10"

// Then you can:
use regex::Regex;
fn printRawString() {
    let default_win_install_path = r"C:\Program Files\Gorillas";
    // You should import the 'regex' crate for Regex::new(). Add this at the top of your file:
    // use regex::Regex;
    let pattern = Regex::new(r"\d+(\.\d+)*");

    println!("{}", default_win_install_path);

    /*
    Raw strings in Rust start with a single quote (') or a double quote (") followed by a number sign (#).
    The number sign (#) indicates that the string is raw, meaning it will not interpret escape sequences like \n.
    The string continues until the same number of number signs (#) is encountered again.
    The following is a raw string that starts with 'r###"'.
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###'):
     */
    println!(
        r###"
 This raw string started with 'r###"'.
 Therefore it does not end until we reach a quote mark ('"')
 followed immediately by three pound signs ('###'):
"###
    );
}

fn print() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);
    println!(
        "In the room the women come and go,
 Singing of Mount Abora"
    );

    println!(
        "It was a bright, cold day in April, and \
 there were four of us—\
 more or less."
    );
}
