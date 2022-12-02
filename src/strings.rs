// types of string
// immutable ,fixed length and
// growable string

pub fn start() {
    let fixed_length_string = "i am fixed";
    let mut growable_string = String::from("i am growable ");

    println!("fixed_length_string = {} ", fixed_length_string);
    println!("growable_string = {} ", growable_string);

    // to get string length
    println!("growable_string length : {}", growable_string.len());

    // adding more data to growable strings
    // push is used to add a single character
    growable_string.push('a');

    // push_str is used to add a longer characters
    growable_string.push_str("string");

    //check if string is empty
    println!("is empty {}", growable_string.is_empty());

    //check if a string contains a string => similar to .includes() in js
    println!("contains grow {}", growable_string.contains("grow"));

    //replace word
    println!(
        "string replaced {}",
        growable_string.replace("string", "hook")
    );

    //loop through strings by white space
    for token in growable_string.split_whitespace() {
        println!("looping string {}", token);
    }

    //create string with a specific capicity
    let mut capacity = String::with_capacity(10);
    capacity.push_str("this is an array with size of 10");
    println!("{}", capacity);

    //assertion testing
    //assert_eq! matches from left to right
    // println!("{}", assert_eq!(12, capacity.capacity()));
}
