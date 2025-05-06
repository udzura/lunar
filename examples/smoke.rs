extern crate lunar_lang;
extern crate purua;

fn main() {
    let source = r#"
        print("Hello, world!")
        local x = 42
        if x > 0 then
            print("x is positive")
        else
            print("x is non-positive")
        end
    "#;

    match lunar_lang::lua::loader::load_string(source) {
        Ok(program) => println!("Parsed program: {:?}", program),
        Err(e) => eprintln!("Error parsing program: {}", e),
    }
}
// This example demonstrates how to load a Lua program from a string, parse it, and print the result.