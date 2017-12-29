fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let x = 3;

    match x {
        3 => {
            println!("This is how it works i think");
            println!("The code is separated into brackets");
        },  
        4 => println!("four"),
        _ => println!("If no others match"),
    }

    let tup: (i32, f64, u8) = (500, 6.453222, 1);
    let (x, y, z) = tup;

    println!("The value of x is {}, y is {}, and z is {}\n", x, y, z);
    println!("Now to test out parameter stuff \n");
    testing_parameters(6, 7);

    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    for element in 1..2{
        println!("Value at location is {}", element);
    }

    let s1 = String::from("This will get cloned");
    let mut s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_owndership();
    change(&mut s2);
    dangle();
    println!("{}", first_word(&s2));
}

fn takes_owndership(){
    println!("I received a string");
}

fn testing_parameters(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string)
}
fn dangle() -> String {
    let s = String::from("hello");
    s
}

//Gets returns a strin slice
fn first_word(s: &String) -> &str {
    //Converts the string object into an array of bytes
    let bytes = s.as_bytes();

    //We can create an iterator over the array of bytes using .iter()
    // .enumerate() returns a tuple which we destructure to get i and &item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}