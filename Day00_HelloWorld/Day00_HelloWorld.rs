use std::io;

fn main(){
    let mut _input_string = String::new();

    match io::stdin().read_line(&mut _input_string) {
        Ok(_n) => {
            println!("Hello, World.");
            println!("{}", _input_string);
        },
        Err(_err) => {},
    }    
}
