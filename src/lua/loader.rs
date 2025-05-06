use purua::parser::{ast, parser, stream::TokenStream};

#[derive(Debug)]
pub struct LuaProgram {
    pub block: ast::Block,
}

pub fn load_file(path: &str) -> Result<LuaProgram, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    load_string(source.as_str())
}

pub fn load_string(source: &str) -> Result<LuaProgram, Box<dyn std::error::Error>> {
    let mut scanner = purua::scanner::Scanner::new(source);
    scanner.scan()?;
    let token_stream = TokenStream::new(scanner.tokens);
    let block = parser::parse(token_stream)?;
    Ok(LuaProgram { block })
}