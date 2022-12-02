pub fn run() {
    println!("hello motherfucker");
    // formatting
    println!("something : {}", 234);
    // passing multiple values
    println!("value: {} and we have value {}", "1", 2);

    //positional argumengts(pass the index of the values)
    println!(
        " {0} is from {1} and  {0} loves to {2}",
        "hycient", "earth", "code"
    );

    // named arguments
    println!(
        "value: {value} and we have value {value2}",
        value = 1,
        value2 = "man!"
    );

    // placehold traits
    println!("binary: {:b} hex : {:x} octal: {:o}", 10, 10, 10);

    // placeholder for debug trait(to print arrays , objects, tuple)

    println!("{:?}", ("hycient", 2, true, 0.4));

    // do basic maths

    println!("20*20/3 = {}", 20 * 20 / 3)
}
