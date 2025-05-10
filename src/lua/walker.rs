use std::collections::HashMap;

use purua::{parser::ast::*, TokenType};

use super::lunarir::*;

#[derive(Debug)]
pub struct Walker {
    pub msg_stack: Vec<LunarIR>,
    pub idx_of_irep: usize,
    pub current_irep: usize,
    pub idx_of_ireps: HashMap<usize, IrepIndices>,
}

#[derive(Debug, Clone)]
pub struct IrepIndices {
    pub locals: usize,
    pub syms: usize,
    pub pool: usize,
}

impl Walker {
    pub fn new() -> Self {
        Walker {
            msg_stack: Vec::new(),
            idx_of_irep: 0,
            current_irep: 0,
            idx_of_ireps: HashMap::from([(
                0,
                IrepIndices {
                    locals: 0,
                    syms: 0,
                    pool: 0,
                },
            )]),
        }
    }

    pub fn push_msg(&mut self, msg: LunarIR) {
        self.msg_stack.push(msg);
    }

    pub fn walk(&mut self, root: &Block) {
        self.walk_block(root);
    }

    pub fn walk_block(&mut self, block: &Block) {
        let chunk = &block.0;
        self.walk_chunk(chunk);
    }

    pub fn walk_chunk(&mut self, chunk: &Chunk) {
        self.push_msg(LunarIR::ChunkStart(self.idx_of_irep));
        let before_irep = self.current_irep;
        self.current_irep = self.idx_of_irep;
        self.idx_of_irep += 1;
        self.idx_of_ireps.insert(
            self.idx_of_irep,
            IrepIndices {
                locals: 0,
                syms: 0,
                pool: 0,
            },
        );
        if self.current_irep != 0 {
            // TODO: replace dummy operand of OP_ENTER
            self.push_msg(LunarIR::Enter(0x40000));
            // dummy locals = 1
            self.idx_of_ireps.get_mut(&self.current_irep).unwrap().locals += 1;
        }

        let statements = &chunk.0;
        for statement in statements {
            self.walk_stat(statement);
        }

        if let Some(last_stat) = &chunk.1 {
            self.walk_laststat(last_stat);
        } else {
            self.push_msg(LunarIR::NoReturn);
        }

        if self.current_irep == 0 {
            self.push_msg(LunarIR::Stop);
        } else {
            self.current_irep = before_irep;
        }

        self.push_msg(LunarIR::ChunkEnd);
    }

    pub fn walk_stat(&mut self, stat: &Stat) {
        match stat {
            Stat::FunctionCall(function_call) => {
                let func_name = &function_call.0;
                let args = &function_call.2;

                // walk_var
                self.walk_prefixexpr(func_name);

                self.push_msg(LunarIR::FunctionCallStart(
                    self.idx_of_ireps[&self.current_irep].syms - 1,
                ));

                self.walk_args(args);

                self.push_msg(LunarIR::FunctionCallEnd);
            },
            Stat::For(_token, expr, expr1, expr2, block) => {
                let begin = self.ensure_expr_as_number(expr) as usize;
                let end = self.ensure_expr_as_number(expr1) as usize;
                let step = match expr2 {
                    Some(expr) => self.ensure_expr_as_number(expr),
                    None => 1.0,
                } as usize;

                self.push_msg(LunarIR::StoreSym(
                    self.idx_of_ireps[&self.current_irep].syms,
                    "each".to_string(),
                ));
                self.push_msg(LunarIR::ForStart(self.idx_of_ireps[&self.current_irep].syms));
                self.idx_of_ireps.get_mut(&self.current_irep).unwrap().syms += 1;

                self.push_msg(LunarIR::ForParam(
                    begin,
                    end,
                    step,
                ));
                // TODO: token is_a varname of each,
                // which can be used inside of this block!
                // TODO: real handle of children reps... :(
                self.push_msg(LunarIR::Block(self.idx_of_irep - 1));
                self.walk_block(block);
                self.push_msg(LunarIR::ForEnd);
            },
            _ => {
                // Handle other types of statements
                panic!("Unsupported statement: {:?}", stat);
            }
        }
    }

    pub fn walk_args(&mut self, args: &Args) {
        match args {
            Args::ArgsString(string) => {
                let content = &string[1..(string.len()-1)];
                self.push_msg(LunarIR::PoolString(
                    self.idx_of_ireps[&self.current_irep].pool,
                    content.to_owned(),
                ));
                self.push_msg(LunarIR::FunctionCallArg(
                    0,
                    LunarValue::String(self.idx_of_ireps[&self.current_irep].pool),
                ));
                self.idx_of_ireps.get_mut(&self.current_irep).unwrap().pool += 1;
            },
            _ => {
                // Handle other types of arguments
                panic!("Unsupported argument: {:?}", args);
            }
        }
    }

    // TODO: replace TryInto?
    pub fn ensure_expr_as_number(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(f) => *f,
            _ => {
                // Handle other types of expressions
                panic!("Unsupported expression: {:?}", expr);
            }
        }
    }

    pub fn walk_prefixexpr(&mut self, prefix_expr: &PrefixExp) {
        match prefix_expr {
            PrefixExp::PrefixVar(var) => {
                // Handle variable prefix expression
                let var = var.as_ref();
                self.walk_var(var);
            },
            _ => {
                // Handle other types of prefix expressions
                panic!("Unsupported prefix expression: {:?}", prefix_expr);
            }
        }
    }

    pub fn walk_var(&mut self, var: &Var) {
        match var {
            Var::VarName(name) => {
                // Handle variable name
                let name = match name.token_type {
                    TokenType::Name => {
                        name.lexeme.clone()
                    },
                    _ => {
                        // Handle other types of variable names
                        panic!("Unsupported variable name: {:?}", name);
                    }
                };
                self.push_msg(LunarIR::StoreSym(
                    self.idx_of_ireps[&self.current_irep].syms,
                    name.clone().try_into().unwrap(),
                ));
                self.idx_of_ireps.get_mut(&self.current_irep).unwrap().syms += 1;
            },
            _ => {
                // Handle other types of variables
                panic!("Unsupported variable: {:?}", var);
            }
        }
    }

    pub fn walk_laststat(&mut self, last_stat: &LastStat) {
        match last_stat {
            LastStat::Return(_ret) => todo!(),
            LastStat::Break => todo!(),
        }
    }
}
