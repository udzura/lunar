extern crate lunar_lang;
extern crate purua;

fn main() {
    let source = r#"
        print("Hello, world!")
    "#;

    match lunar_lang::lua::loader::load_string(source) {
        Ok(program) => {
            // println!("Parsed program: {:?}", &program);
            let mut walker = lunar_lang::lua::walker::Walker::new();
            walker.walk(&program.block);
            for msg in walker.msg_stack {
                println!("MSG: {}", msg);
            }
        },
        Err(e) => eprintln!("Error parsing program: {}", e),
    }
}
// This example demonstrates how to load a Lua program from a string, parse it, and print the result.