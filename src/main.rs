mod parser;

use parser::compile_asm2ms;


fn main() {
    match compile_asm2ms() {
        Ok(_) => (),
        Err(error_message) => println!("{error_message:?}")
    };

    println!("Done");
}
