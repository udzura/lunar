use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::rite::bytecode::*;
use crate::lua::lunarir::*;

#[derive(Debug)]
pub struct IrepBase {
    pub locals: usize,
    pub regs: usize,
    pub rep_len: usize,
    pub chandlers: usize, // TODO: fixed to 0; not yet implemented
    pub syms: HashMap<usize, String>,
    pub pool: HashMap<usize, String>,
    pub insn: Vec<Bytecode>,

    pub parent: Option<Rc<RefCell<IrepBase>>>,
}

impl IrepBase {
    pub fn new() -> Rc<RefCell<Self>>{
        let base = IrepBase {
            locals: 0,
            regs: 0,
            rep_len: 0,
            chandlers: 0,
            syms: HashMap::new(),
            pool: HashMap::new(),
            insn: Vec::new(),
            parent: None,
        };
        Rc::new(RefCell::new(base))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransformState {
    Top,
    InFor {
        reg: usize,
        sym: usize,
    },
    InFuncall {
        sym: usize,
        reg: usize,
        argsize: usize,
    },
}

pub fn transform(lunar_ir: &[LunarIR]) -> Vec<Rc<RefCell<IrepBase>>> {
    let mut reps = Vec::new();
    let mut current: Rc<RefCell<IrepBase>> = IrepBase::new();
    reps.push(current.clone());
    let mut state = TransformState::Top;
    let mut old_states = Vec::new();

    for msg in lunar_ir {
        match msg {
            LunarIR::ChunkStart(i) => {
                if *i != 0 {
                    let new_irep = IrepBase::new();
                    current.borrow_mut().rep_len += 1;
                    new_irep.borrow_mut().parent = Some(current.clone());
                    reps.push(new_irep.clone());
                    current = new_irep;
                }

                old_states.push(state);
                state = TransformState::Top;
            },
            LunarIR::ChunkEnd => {
                let current_ = current.clone();
                let old = current_.borrow_mut();
                if let Some(p) = old.parent.clone() {
                    current = p;
                }
                state = old_states.pop().unwrap();
            },
            LunarIR::Local(_) => todo!(),
            LunarIR::ForStart(sym) => {
                old_states.push(state);
                state = TransformState::InFor { reg: 0, sym: *sym };
            },
            LunarIR::ForParam(start, end, step) => {
                if let TransformState::InFor{ reg: _, sym } = state {
                    let mut pushed = 0;
                    let mut first_reg = -1;
                    for i in (*start..=*end).step_by(*step) {
                        current.borrow_mut().regs += 1;

                        let reg = current.borrow().regs;
                        current.borrow_mut().insn.push(Bytecode {
                            op: OpCode::LOADI,
                            operand: Operand::BB(reg as u8, i as u8)
                        });

                        pushed += 1;

                        if first_reg == -1 {
                            first_reg = reg as isize;
                        }
                    }
                    current.borrow_mut().insn.push(Bytecode {
                        op: OpCode::ARRAY,
                        operand: Operand::BB(first_reg as u8, pushed as u8)
                    });

                    state = TransformState::InFor { reg: first_reg as usize, sym };
                } else {
                    panic!("Invalid state: expected InFor context");
                }
            },
            LunarIR::ForEnd => {
                if let TransformState::InFor{ reg, sym } = state {
                    current.borrow_mut().insn.push(Bytecode {
                        op: OpCode::SENDB,
                        operand: Operand::BBB(reg as u8, sym as u8, 0u8)
                    });

                    state = old_states.pop().unwrap();
                } else {
                    panic!("Invalid state: expected InFor context");
                }
            },
            LunarIR::Enter(eval) => {
                current.borrow_mut().insn.push(Bytecode {
                    op: OpCode::ENTER,
                    operand: Operand::W(*eval),
                });
                // TODO: get argsize from ENTER valus
                current.borrow_mut().regs += 1;
                current.borrow_mut().locals += 1;
                // set regs = locals + 1
                current.borrow_mut().regs += 1;
            },
            LunarIR::StoreSym(idx, name) => {
                current.borrow_mut().syms.insert(*idx, name.clone());
            },
            LunarIR::FunctionCallStart(sym) => {
                old_states.push(state);
                current.borrow_mut().regs += 1;
                let reg = current.borrow().regs;
                state = TransformState::InFuncall {
                    sym: *sym,
                    reg,
                    argsize: 0,
                };
            },
            LunarIR::FunctionCallArg(_, lunar_value) => {
                if let TransformState::InFuncall{ sym, reg, argsize } = state {
                    current.borrow_mut().regs += 1;
                    let arg_reg = current.borrow().regs;
                    match lunar_value {
                        LunarValue::Nil => {
                            current.borrow_mut().insn.push(Bytecode {
                                op: OpCode::LOADNIL,
                                operand: Operand::B(arg_reg as u8),
                            });
                        },
                        LunarValue::Boolean(b) => {
                            if *b {
                                current.borrow_mut().insn.push(Bytecode {
                                    op: OpCode::LOADT,
                                    operand: Operand::B(arg_reg as u8),
                                });
                            } else {
                                current.borrow_mut().insn.push(Bytecode {
                                    op: OpCode::LOADF,
                                    operand: Operand::B(arg_reg as u8),
                                });
                            }
                        },
                        LunarValue::Number(n) => {
                            current.borrow_mut().insn.push(Bytecode {
                                op: OpCode::LOADI,
                                operand: Operand::BB(arg_reg as u8, *n as u8),
                            });
                        },
                        LunarValue::String(pool_idx) => {
                            current.borrow_mut().insn.push(Bytecode {
                                op: OpCode::STRING,
                                operand: Operand::BB(arg_reg as u8, *pool_idx as u8),
                            });
                        },
                    }
                    state = TransformState::InFuncall { sym, reg, argsize: argsize + 1 };
                } else {
                    panic!("Invalid state: expected InFuncall context");
                }
            },
            LunarIR::FunctionCallEnd => {
                if let TransformState::InFuncall{ sym, reg, argsize } = state {
                    current.borrow_mut().insn.push(Bytecode {
                        op: OpCode::SSEND,
                        operand: Operand::BBB(reg as u8, sym as u8, argsize as u8)
                    });
                    state = old_states.pop().unwrap();
                } else {
                    panic!("Invalid state: expected InFuncall context");
                }
            },
            LunarIR::PoolString(idx, value) => {
                current.borrow_mut().pool.insert(*idx, value.clone());
            },
            LunarIR::Block(b) => {
                if let TransformState::InFor{ reg, sym: _ } = state {
                    current.borrow_mut().insn.push(Bytecode {
                        op: OpCode::BLOCK,
                        operand: Operand::BB(reg as u8 + 1, *b as u8)
                    });
                } else {
                    current.borrow_mut().regs += 1;
                    let reg = current.borrow().regs;
                    current.borrow_mut().insn.push(Bytecode {
                        op: OpCode::BLOCK,
                        operand: Operand::BB(reg as u8, *b as u8)
                    });
                }
            },
            LunarIR::NoReturn => {
                current.borrow_mut().regs += 1;
                let reg = current.borrow().regs;
                current.borrow_mut().insn.push(Bytecode {
                    op: OpCode::LOADNIL,
                    operand: Operand::B(reg as u8),
                });
                current.borrow_mut().insn.push(Bytecode {
                    op: OpCode::RETURN,
                    operand: Operand::B(reg as u8),
                });
            },
            LunarIR::Stop => {
                current.borrow_mut().insn.push(Bytecode {
                    op: OpCode::STOP,
                    operand: Operand::Z,
                });
            },
        }
    }

    loop {
        let target = current.clone();
        let refirep = target.borrow();
        if let Some(parent) = refirep.parent.clone() {
            current = parent.clone();
        } else {
            break;
        }
    }
    reps
}