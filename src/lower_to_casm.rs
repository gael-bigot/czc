use crate::ast::*;
use crate::casm::{CasmInstruction, Operand};
use std::collections::HashMap;

pub struct Compiler {
    code_elements: Vec<CodeElement>,
    casm_instructions: Vec<CasmInstruction>,
    ap_minus_fp: i32,
    local_variables: HashMap<String, i32>,
    //current_function_name: Option<String>,
}

impl Compiler {
    pub fn new(code_elements: Vec<CodeElement>) -> Self {
        Self { code_elements, casm_instructions: Vec::new(), ap_minus_fp: 0, local_variables: HashMap::new() }
    }

    pub fn compile(&mut self) -> Vec<CasmInstruction> {
        for code_element in self.code_elements.clone() {
            self.compile_code_element(code_element);
        }
        self.casm_instructions.clone()
    }

    pub fn evaluate_expr(&mut self, expr: Expr) -> Operand {
        match expr.expr_type {
            ExprType::IntegerLiteral => {
                Operand::Int(expr.token.unwrap().lexeme.parse::<i32>().unwrap())
            }
            ExprType::Add => {
                let left = self.evaluate_expr(*expr.left.unwrap());
                let right = self.evaluate_expr(*expr.right.unwrap());
                let op = CasmInstruction::Add {
                    left: Operand::DerefFp(self.ap_minus_fp),
                    op1: left,
                    op2: right,
                };
                self.casm_instructions.push(op);
                self.ap_minus_fp += 1;
                Operand::DerefFp(self.ap_minus_fp-1)
            }
            ExprType::Sub => {
                let left = self.evaluate_expr(*expr.left.unwrap());
                let right = self.evaluate_expr(*expr.right.unwrap());
                let op = CasmInstruction::Add {
                    left: left,
                    op1: right,
                    op2: Operand::DerefFp(self.ap_minus_fp),
                };
                self.casm_instructions.push(op);
                self.ap_minus_fp += 1;
                Operand::DerefFp(self.ap_minus_fp-1)
            }

            ExprType::Mul => {
                let left = self.evaluate_expr(*expr.left.unwrap());
                let right = self.evaluate_expr(*expr.right.unwrap());
                let op = CasmInstruction::Mul {
                    left: Operand::DerefFp(self.ap_minus_fp),
                    op1: left,
                    op2: right,
                };
                self.casm_instructions.push(op);
                self.ap_minus_fp += 1;
                Operand::DerefFp(self.ap_minus_fp-1)
            }

            ExprType::FunctionCall => {
                let func_name = expr.ident.unwrap().token.lexeme;
                let mut arg_refs = Vec::new();
                for arg in expr.paren_args {
                    let arg_ref = match arg {
                        ExprAssignment::Expr(expr) => self.evaluate_expr(expr),
                        ExprAssignment::Assign(ident, expr) => todo!()
                    };
                    arg_refs.push(arg_ref);
                }
                for arg_ref in arg_refs {
                    // once args are calculated, we can push them to stack
                    let instr = CasmInstruction::Set {
                        left: Operand::DerefFp(self.ap_minus_fp),
                        op: arg_ref,
                    };
                    self.casm_instructions.push(instr);
                    self.ap_minus_fp += 1;
                }
                
                // pushin pc to top of Stack
                let instr = CasmInstruction::Set {
                    left: Operand::DerefFp(self.ap_minus_fp),
                    op: Operand::DerefPc(3),
                };
                self.casm_instructions.push(instr);
                self.ap_minus_fp += 1;
                // setting fp to top of Stack
                let instr = CasmInstruction::IncrFp(self.ap_minus_fp);
                self.casm_instructions.push(instr);
                self.ap_minus_fp += 1;
                // calling function
                let instr = CasmInstruction::Call(func_name);
                self.casm_instructions.push(instr);
                self.ap_minus_fp = 0;
                Operand::DerefFp(-2)
            }

            ExprType::Identifier => {
                let ident = expr.ident.unwrap().token.lexeme;
                Operand::DerefFp(self.local_variables[&ident])
            }

            _ => todo!(),
        }
    }


    pub fn compile_function(&mut self, name: Identifier, args: Vec<Identifier>, body: Vec<CodeElement>) {
        self.local_variables.clear();
        self.ap_minus_fp = 0;

        self.casm_instructions.push(CasmInstruction::Label(name.token.lexeme));

        for (i, arg) in args.iter().enumerate() {
            self.local_variables.insert(arg.token.lexeme.clone(), -(args.len() as i32 + 1) + i as i32);
        }
        for code_element in body {
            self.compile_code_element(code_element);
        }
    }

    pub fn compile_code_element(&mut self, code_element: CodeElement) {
        match code_element {
            CodeElement::Reference(ident, expr) => {
                let value = self.evaluate_expr(expr);
                match value {
                    Operand::Int(n) => {
                        let instr = CasmInstruction::Set {
                            left: Operand::DerefFp(self.ap_minus_fp),
                            op: Operand::Int(n),
                        };
                        self.casm_instructions.push(instr);
                        self.ap_minus_fp += 1;
                        self.local_variables.insert(ident.token.lexeme, self.ap_minus_fp-1);
                    }
                    Operand::DerefFp(offset) => {
                        self.local_variables.insert(ident.token.lexeme, offset);
                    }
                    _ => todo!(),
                }
            }
            CodeElement::Return(expr) => {
                // calculating return value
                let value = self.evaluate_expr(expr);
                // putting return value to top of stack
                match value {
                    Operand::Int(n) => {
                        let instr = CasmInstruction::Set {
                            left: Operand::DerefFp(self.ap_minus_fp),
                            op: Operand::Int(n),
                        };
                        self.casm_instructions.push(instr);
                        self.ap_minus_fp += 1;
                    }
                    Operand::DerefFp(offset) => {
                        let instr = CasmInstruction::Set {
                            left: Operand::DerefFp(self.ap_minus_fp),
                            op: Operand::DerefFp(offset),
                        };
                        self.casm_instructions.push(instr);
                        self.ap_minus_fp += 1;
                    }
                    _ => todo!(),
                }
                // pushing adress return value to stack
                let instr = CasmInstruction::Set {
                    left: Operand::DerefFp(self.ap_minus_fp),
                    op: Operand::DerefFp(-1),
                };
                self.casm_instructions.push(instr);
                self.ap_minus_fp += 1;
                // setting fp to top of stack
                let instr = CasmInstruction::IncrFp(self.ap_minus_fp);
                self.casm_instructions.push(instr);
                self.ap_minus_fp += 1;
                self.casm_instructions.push(CasmInstruction::Ret);
            }
            CodeElement::Function(name, args, body) => {
                self.compile_function(name, args, body);
            }
            CodeElement::CompoundAssertEqual(expr1, expr2) => {
                let value1 = self.evaluate_expr(expr1);
                let value2 = self.evaluate_expr(expr2);
                let instr = CasmInstruction::Set {
                    left: value1,
                    op: value2,
                };
                self.casm_instructions.push(instr);
            }
            CodeElement::If(expr, body, else_body) => {
                assert!(matches!(expr.expr_type, ExprType::Neq));
                let test_value = self.evaluate_expr(Expr::new_binary(ExprType::Sub, *expr.left.unwrap(), *expr.right.unwrap()));

                match test_value {
                    Operand::Int(n) => unreachable!(),
                    Operand::DerefFp(offset) => {
                        let instr = CasmInstruction::JmpIfNeq(0, Operand::DerefFp(offset));
                        self.casm_instructions.push(instr);
                        // saving state
                        let instruction_number = self.casm_instructions.len() as i32;
                        let current_ap_minus_fp = self.ap_minus_fp;
                        // compiling else body
                        else_body.iter().for_each(|code_element| {
                            self.compile_code_element(code_element.clone());
                        });
                        self.ap_minus_fp = current_ap_minus_fp;
                        let else_body_size = self.casm_instructions.len() as i32 - instruction_number;
                        // compiling else body
                        body.iter().for_each(|code_element| {
                            self.compile_code_element(code_element.clone());
                        });
                        // updating jump instruction
                        self.casm_instructions[instruction_number as usize - 1] = CasmInstruction::JmpIfNeq(else_body_size+1, Operand::DerefFp(offset));
                        
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}