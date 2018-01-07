extern crate yadql;

use yadql::Database;

fn main() {
    let db = Database::connect("");
    println!("A Test YADQL Client");
    println!("First, we'll insert the value 'x': 'y'.");
    println!("Insert 'x' 'y';");
    println!(db::execute("Insert 'x' 'y';"));
    println!(db::execute("Read 'x'"));
    println!("Then we'll update it to 'x':'z'.");
    println!("Update 'x' 'z';");
    println!(db::execute("Update 'x' 'z';"));
    println!(db::execute("Read 'x'"));
    println!("Then we'll delete x altogether.");
    println!("Delete 'x';");
    println!(db::execute("Delete 'x';"));
    println!("This proves that the library errors properly when reading a non-existent value.")
    println!(db::execute("Read 'x'"));
}
