
/*
    Set of cargo commands
    cargo new - create a new cargo project
    cargo build - build project(compile and generate exec)
    cargo check - compile without generating exec

*/     

// import module (when in same dir, can omit the path)
// #[path ="./formatted_print.rs"] mod formatted_print;




/*
    This method demonstrates formatted string
*/
fn formatted_string() {
    // {} will be replaced by the args and connverted to string
    println!("My age is: {}", 23);

    // specifying positional args: 0 for first, 1 for second and so on
    println!("Your name is {0} and age is {1}. Hello {0}", "John", 24);

    // args can be named as well
    println!("{name}, born on {day}.",
                name= "Solomon Grundy",
                day = "Monday");

    // format specifier: {:fmt}
    println!("This is decimal repr:     {}", 23567);
    println!("This is binary repr:      {:b}", 23567);
    println!("This is octal repr:       {:o}", 23567);
    println!("This is hexdecimal repr:  {:x}", 23567);
    println!("This is exp repr:         {:e}", 23567);


    // fill/alignment
    println!("The name will have 5 widespace: {:5}!","John"); // min width a param should take(here it will take 5 space min : string sat the beginning)
    println!("The name will have 5 widespace: {:>5}!","x"); // padding left
    println!("The name will have 5 widespace: {:<5}!","x"); // padding right
    println!("The value will have 5 widespace: {:0<5}!","x"); // 0 fills the space
    println!("The value will have {width} widespace: {name:0<width$}!",name = "x", width = 7); // using dynamic vars


    //floating precision
    let float_num:f32 = 2.3435546;
    println!("Number {num} upto 4 precision: {num:.4}", num = float_num);

    // formatted text to a string var
    let formatted_string:String = format!("Hello there: {name:30}\nyou've been on this earth since the year {year} at {seconds:.2}", 
    name = "John",
    year = 2000,
    seconds = 2345.456/60.0);
    println!("Formatted String: {}",formatted_string);




        
}


fn main() {
    // this is a macro: line is replaced by the code during compilation
    // println!("Hello, world!");

    formatted_string();

}