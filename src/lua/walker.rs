use purua::parser::ast::*;

#[derive(Debug)]
pub struct Walker {
    pub msg_stack: Vec<String>,
}

impl Walker {
    pub fn new() -> Self {
        Walker {
            msg_stack: Vec::new(),
        }
    }

    pub fn push_msg(&mut self, msg: String) {
        self.msg_stack.push(msg);
    }

    pub fn walk(&mut self, root: &Block) {
        let chunk = &root.0;
        let statements = &chunk.0;
        for statement in statements {
            self.walk_stat(statement);
        }
        if let Some(last_stat) = &chunk.1 {
            self.walk_laststat(last_stat);
        }
    }

    pub fn walk_stat(&mut self, stat: &Stat) {
        match stat {
            Stat::Assign(var_list, expr_list) => todo!(),
            Stat::FunctionCall(function_call) => {
                // Handle function call
                let func_name = &function_call.0;
                let args = &function_call.2;
                self.push_msg(format!("Function call: {:?} with args: {:?}", func_name, args));
            },
            Stat::Do(block) => todo!(),
            Stat::While(expr, block) => todo!(),
            Stat::Repeat(expr, block) => todo!(),
            Stat::If(expr, block, items, block1) => todo!(),
            Stat::For(token, expr, expr1, expr2, block) => todo!(),
            Stat::ForIn(name_list, expr_list, block) => todo!(),
            Stat::Function(func_name, func_body) => todo!(),
            Stat::LocalFunction(token, func_body) => todo!(),
            Stat::LocalDeclVar(name_list, expr_list) => todo!(),
        }
    }

    pub fn walk_laststat(&mut self, last_stat: &LastStat) {
        match last_stat {
            LastStat::Return(ret) => todo!(),
            LastStat::Break => todo!(),
        }
    }
}

