// variables are immutable by default
//rust is a block scoped language
// variables holds primitive data types or references

//pub makes the var publicly accessible
pub fn run() {
    let name = "hycient";
    let age = 34;

    // to make a variable reassignable you add "mut"
    let mut mutable_string = "will be chnaged soon";

    mutable_string = "i have been changed";

    println!("{} is {}", name, age);

    // when using const you must add data type

    const ID: i32 = 001;
    println!("ID: {}", ID);

    //asign multiple values

    let (first1, second1) = ("first", "second");
}
