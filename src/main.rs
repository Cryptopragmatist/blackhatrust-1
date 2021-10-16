use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    //Returns the arguments that this program was started with (normally passed via the command line)
    //.collect method put to vector array of strings
    //env::args() calls the method args from this module
    
    println!("{} arguments passed", args.len());
    //prints out the # of arguments that this program was started with (cargo run in the terminal), .len() counts the args
    
}
