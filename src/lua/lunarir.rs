#[derive(Debug, Clone, Copy)]
pub enum LunarValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(usize),
    // TODO: add more types
}

#[derive(Debug, Clone)]
pub enum LunarIR {
    ChunkStart(usize),
    ChunkEnd,
    Local(usize),
    ForStart,
    ForParam(usize, usize, usize),
    ForEnd,
    Enter(u32),
    StoreSym(usize, String),
    FunctionCallStart(usize),
    FunctionCallArg(usize, LunarValue),
    FunctionCallEnd,
    PoolString(usize, String),
    Block(usize),
    NoReturn,
    Stop,
}