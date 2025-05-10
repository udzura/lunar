extern crate lunar_lang;
use clap::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut command = command!()
        .name("lunar")
        .version(VERSION)
        .about("Lunar: A Lua-to-mruby compiler")
        .subcommand(
            Command::new("compile")
                .about("Compile a Lua source file to an mruby binary")
                .arg(arg!(-o --output <OUTPUT> "Output mruby binary file"))
                .arg(arg!(--debug "Enable debug information"))
                .arg(arg!([lua_script] "Lua source file to compile")),
        );
    let matches = command.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let debug = matches.get_flag("debug");
        let lua_path = matches.get_one::<String>("lua_script").expect("require lua script");
        let lua_path = lua_path.to_owned();
        let output = if let Some(value) = matches.get_one::<String>("output") {
            value.to_owned()
        } else {
            lua_path.replace(".lua", ".mrb")
        };

        if debug {
            eprintln!("Debug mode enabled");
        }

        match lunar_lang::lua::loader::load_file(&lua_path) {
            Ok(program) => {
                let mut walker = lunar_lang::lua::walker::Walker::new();
                walker.walk(&program.block);
                if debug {
                    for (i, msg) in walker.msg_stack.iter().enumerate() {
                        eprintln!("LUNARIR: {:<04}: {:?}", i, msg);
                    }
                }

                let mruby = lunar_lang::rite::transformer::transform(&walker.msg_stack);
                if debug {
                    for (i, rep) in mruby.iter().enumerate() {
                        eprintln!("IREP: {:<04}: rlen = {}", i, rep.borrow().rep_len);
                        eprintln!("IREP: {:<04}: pool = {:?}", i, &rep.borrow().pool);
                        eprintln!("IREP: {:<04}: syms = {:?}", i, &rep.borrow().syms);
                        eprintln!("IREP: {:<04}: nregs = {:?}", i, &rep.borrow().regs);
                        eprintln!("IREP: {:<04}: nlocals = {:?}", i, &rep.borrow().locals);
                        for (j, msg) in rep.borrow().insn.iter().enumerate() {
                            eprintln!("IREP: {:<04}:{:<04} {:?}", i, j, msg);
                        }
                    }
                }

                let mut packer = lunar_lang::rite::packer::RitePacker::new();
                match packer.pack(&mruby) {
                    Ok(_) => if debug {
                        eprintln!("Packed binary size: {} bytes", packer.buf.len());
                    }
                    Err(e) => eprintln!("Error packing: {}", e),
                }

                match packer.write_to_file(&output) {
                    Ok(_) => if debug {
                        eprintln!("Packed binary written to {}", output)
                    },
                    Err(e) => eprintln!("Error writing to file: {}", e),
                }
            }
            Err(e) => eprintln!("Error parsing program: {}", e),
        }
    } else {
        command.print_help().unwrap();
    }
}
