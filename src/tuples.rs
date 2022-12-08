//tuples groups values of different types and it can have a max value of 12

pub fn init() {
    let person: (&str, &str, i8) = ("hycient", "laptops", 18);

    // how to access a tuple

    println!("{} owns {} {}", person.0, person.2, person.1);
}
