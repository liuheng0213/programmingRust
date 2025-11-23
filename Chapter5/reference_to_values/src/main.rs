fn main() {
    // shared_reference_demo();
    mutable_reference_demo();
}

fn mutable_reference_demo() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    //To modify the table, we need a mutable reference.
    //We can get one by using &mut when calling the function:
    add_work(&mut table, "Caravaggio", "Judith Beheading Holofernes");

    assert_eq!(
        table["Caravaggio"],
        vec![
            "The Musicians",
            "The Calling of St. Matthew",
            "Judith Beheading Holofernes"
        ]
    );
}

fn add_work(table: &mut Table, artist: &str, work: &str) {
    if let Some(works) = table.get_mut(artist) {
        works.push(work.to_string());
    }
}

use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;
fn shared_reference_demo() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    //    show(table) causes a compile-time error.
    //In particular, HashMap is not Copy—  it can’t be,
    //since it owns a dynamically allocated table.
    //So when the program calls show(table),
    //the whole structure gets moved to the function,
    //leaving the variable table uninitialized.

    //In fact, if we look into the definition of show,
    //the outer for loop takes ownership of the hash table and consumes it entirely;
    //and the inner for loop does the same to each of the vectors.
    //(We saw this behavior earlier,
    //in the “liberté, égalité, fraternité” example.)
    //Because of move semantics, we’ve completely destroyed the entire structure simply by trying to print it out. Thanks, Rust!

    //The printing function in our example doesn’t need to modify the table, just read its contents.
    //So the caller should be able to pass it a shared reference to the table,
    //as follows:

    //The type of show’s parameter table has changed from Table to &Table:
    // instead of passing the table by value (and hence moving ownership into the function), 
    //we’re now passing a shared reference. 
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
