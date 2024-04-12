// mod person;
// // pub keyword let's us set this function to be public so other files can use it
// pub fn hello() {
//     // "let" keyword tells us we're making a variable
//     // "mut" keyword means this variable will be mutable, meaning it can change
//     let mut input: String = String::new();
//     // println! is a macro that prints text to console
//     println!("Enter your name :");
//     // Our access to console input
//     let std_in: std::io::Stdin = std::io::stdin();

//     let person: Person = new Person(input);
//     //reads from stdin and writes to mutable variable "input", returns value of how many bytse read
//     let bytes: usize = std_in.read_line(&mut input).unwrap();
    
//     println!("Hello, {}", input);
//     println!("no. of bytes read, {}", bytes)
// }

