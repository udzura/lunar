extern crate lunar_lang;
extern crate purua;

fn main() {
    let source = r#"
for i = 1, 5 do
   print "hello, world\n"
end
"#;
    // let source = r#"print "hello, world\n""#;

    match lunar_lang::lua::loader::load_string(source) {
        Ok(program) => {
            println!("Parsed program: {:?}", &program);
            let mut walker = lunar_lang::lua::walker::Walker::new();
            walker.walk(&program.block);
            for (i, msg) in walker.msg_stack.iter().enumerate() {
                println!("MSG: {:<04}: {:?}", i, msg);
            }

            let mruby = lunar_lang::rite::transformer::transform(&walker.msg_stack);
            for (i, rep) in mruby.iter().enumerate() {
                println!("IREP: {:<04}: rlen = {}", i, rep.borrow().rep_len);
                println!("IREP: {:<04}: syms = {:?}", i, &rep.borrow().syms);
                println!("IREP: {:<04}: pool = {:?}", i, &rep.borrow().pool);
                println!("IREP: {:<04}: nregs = {:?}", i, &rep.borrow().regs);
                println!("IREP: {:<04}: nlocals = {:?}", i, &rep.borrow().locals);
                for (j, msg) in rep.borrow().insn.iter().enumerate() {
                    println!("IREP: {:<04}:{:<04} {:?}", i, j, msg);
                }
            }

            let mut packer = lunar_lang::rite::packer::RitePacker::new();
            match packer.pack(&mruby) {
                Ok(_) => {
                    // let packed = &packer.buf;
                }
                Err(e) => eprintln!("Error packing: {}", e),
            }

            match packer.write_to_file("test.mrb") {
                Ok(_) => println!("Packed binary written to ./test.mrb"),
                Err(e) => eprintln!("Error writing to file: {}", e),
            }
        },
        Err(e) => eprintln!("Error parsing program: {}", e),
    }
}
// This example demonstrates how to load a Lua program from a string, parse it, and print the result.