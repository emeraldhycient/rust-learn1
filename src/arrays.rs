// array length is fixed where elemts are of same type

pub fn start() {
    // first index in the array is data type , second value is length
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //print the array

    println!("{:?}", numbers);

    // when u need the array to be mutable
    let mut stringarray: [&str; 5] = ["2", "3", "4", "5", "6"];
    stringarray[3] = "man";

    println!("{:?}", stringarray);

    // to get the amount of memory it takes

    println!(
        "size of memory taken up by this array : {}bytes",
        std::mem::size_of_val(&stringarray)
    );
}
