use crate::ast::*;
use crate::casm::{CasmInstruction, Operand};
use std::collections::HashMap;

pub struct Compiler {
    code_elements: Vec<CodeElement>,
    casm_instructions: Vec<CasmInstruction>,
    local_variables: HashMap<String, i32>,
    size_of_locals: u64,
    current_local_offset: u64,
    //current_function_name: Option<String>,
}

impl Compiler {
    pub fn new(code_elements: Vec<CodeElement>) -> Self {
        Self { code_elements, casm_instructions: Vec::new(), local_variables: HashMap::new(), size_of_locals: 0, current_local_offset: 0 }
    }

    pub fn compile(&mut self) -> Vec<CasmInstruction> {
        for code_element in self.code_elements.clone() {
            self.compile_code_element(code_element);
        }
        self.casm_instructions.clone()
    }

    // pushes litteral on stack and returns ap offset (ie 1)
    fn compile_int_literal(&mut self, expr: Expr) -> i32 {
        assert!(matches!(expr.expr_type, ExprType::IntegerLiteral));
        self.casm_instructions.push(CasmInstruction::Set {
            left: Operand::DerefAp(0),
            op: Operand::Int(expr.token.unwrap().lexeme.parse::<u64>().unwrap()),
            incr_ap: true,
        });
        return 1
    }

    
    fn compile_add(&mut self, expr: Expr) -> i32 {
        assert!(matches!(expr.expr_type, ExprType::Add));
        let left_offset = self.compile_expr(*expr.left.unwrap());
        let right_offset = self.compile_expr(*expr.right.unwrap());
        self.casm_instructions.push(CasmInstruction::Add {
            left: Operand::DerefAp(0),
            op1: Operand::DerefAp(-1 - right_offset),
            op2: Operand::DerefAp(-1),
        });
        left_offset + right_offset + 1
    }

    fn compile_sub(&mut self, expr: Expr) -> i32 {
        let left_offset = self.compile_expr(*expr.left.unwrap());
        let right_offset = self.compile_expr(*expr.right.unwrap());
        self.casm_instructions.push(CasmInstruction::Add {
            left: Operand::DerefAp(-1 - right_offset),
            op1: Operand::DerefAp(0),
            op2: Operand::DerefAp(-1),
        });
        left_offset + right_offset + 1
    }

    fn compile_mul(&mut self, expr: Expr) -> i32 {
        let left_offset = self.compile_expr(*expr.left.unwrap());
        let right_offset = self.compile_expr(*expr.right.unwrap());
        self.casm_instructions.push(CasmInstruction::Mul {
            left: Operand::DerefAp(0),
            op1: Operand::DerefAp(-1 - right_offset),
            op2: Operand::DerefAp(-1),
        });
        left_offset + right_offset + 1
    }

    fn compile_function_call(&mut self, expr: Expr) -> i32 {
        let func_name = expr.ident.unwrap().token.lexeme;
        let mut arg_offsets = Vec::new();
        // evaluating each argument and storing the ap offset
        for arg in expr.paren_args {
            let arg_offset = match arg {
                ExprAssignment::Expr(expr) => self.compile_expr(expr),
                ExprAssignment::Assign(ident, expr) => todo!()
            };
            arg_offsets.push(arg_offset);
        }
        // Calculate sum of all elements after each index, ie cumulative offset to reach each arg
        let mut total = 0;
        for arg_offset in arg_offsets.iter_mut().rev() {
            let current = *arg_offset;
            *arg_offset = total;
            total += current;
            
        }
        // Push args to stack
        for (i, arg_offset) in arg_offsets.iter().enumerate() {
            let instr = CasmInstruction::Set {
                left: Operand::DerefAp(0),
                op: Operand::DerefAp(-1- arg_offset - i as i32), // i accounts for the fact that pushing each argument further increases the ap offset
                incr_ap: true,
            };
            self.casm_instructions.push(instr);
        }
        // calling function
        let instr = CasmInstruction::Call(func_name);
        self.casm_instructions.push(instr);
        // return value is at top of stack
        1
    }

    // pushes local variable on stack and returns ap offset (ie 1)
    fn compile_identifier(&mut self, expr: Expr) -> i32 {
        let ident = expr.ident.unwrap().token.lexeme;
        self.casm_instructions.push(CasmInstruction::Set {
            left: Operand::DerefAp(0),
            op: Operand::DerefFp(self.local_variables[&ident] as i32),
            incr_ap: true,
        });
        return 1;
    }

    pub fn compile_expr(&mut self, expr: Expr) -> i32 {
        // compiles an expression and returns the total number of times ap was incremented
        // operations are done at the top of the stack
        // note that for ease of implementation, integers as well as references to variables are all pushed on stack
        // which creates a lot of unecessary copies and instructions
        match expr.expr_type {
            ExprType::IntegerLiteral => self.compile_int_literal(expr),
            ExprType::Add => self.compile_add(expr),
            ExprType::Sub => self.compile_sub(expr),
            ExprType::Mul => self.compile_mul(expr),
            ExprType::FunctionCall => self.compile_function_call(expr),
            ExprType::Identifier => self.compile_identifier(expr),

            _ => todo!(),
        }
    }


    pub fn compile_function(&mut self, name: Identifier, args: Vec<Identifier>, body: Vec<CodeElement>) {
        self.local_variables.clear();
        self.size_of_locals = 0;
        self.current_local_offset = 0;
        // counting number of local declarations
        for code_element in body.clone() {
            match code_element {
                CodeElement::LocalVar(ident, expr) => {
                    self.size_of_locals += 1;
                }
                _ => {}
            }
        }

        self.casm_instructions.push(CasmInstruction::Label(name.token.lexeme));

        for (i, arg) in args.iter().enumerate() {
            self.local_variables.insert(arg.token.lexeme.clone(), -(args.len() as i32 + 2) + i as i32);
        }
        for code_element in body {
            self.compile_code_element(code_element);
        }
    }

    fn compile_local_var(&mut self, ident: Identifier, expr: Option<Expr>) {
        self.local_variables.insert(ident.token.lexeme, self.current_local_offset as i32);
        self.current_local_offset += 1;
        
        match expr {
            Some(expr) => {
                let _ = self.compile_expr(expr);
                let instr = CasmInstruction::Set {
                    left: Operand::DerefFp(self.current_local_offset as i32 -1),
                    op: Operand::DerefAp(-1),
                    incr_ap: false,
                };
                self.casm_instructions.push(instr);
            }
            None => {}
        }
    }

    fn compile_return(&mut self, expr: Expr) {
        // calculating return value
        // it is automatically at top of stack
        let _ = self.compile_expr(expr);
        
        self.casm_instructions.push(CasmInstruction::Ret);
    }

    fn compile_compound_assert_equal(&mut self, expr1: Expr, expr2: Expr) {
        let value1 = self.compile_expr(expr1);
        let value2 = self.compile_expr(expr2);
        let instr = CasmInstruction::Set {
            left: Operand::DerefAp(0),
            op: Operand::DerefAp(- value2),
            incr_ap: false,
        };
        self.casm_instructions.push(instr);
    }

    fn compile_if(&mut self, expr: Expr, body: Vec<CodeElement>, else_body: Vec<CodeElement>) {
        assert!(matches!(expr.expr_type, ExprType::Neq));
        let _ = self.compile_expr(Expr::new_binary(ExprType::Sub, *expr.left.unwrap(), *expr.right.unwrap()));

        let instr = CasmInstruction::JmpIfNeq(0, Operand::DerefAp(-1));
        self.casm_instructions.push(instr);
        // saving state
        let instruction_number = self.casm_instructions.len() as i32;
        // compiling else body
        else_body.iter().for_each(|code_element| {
            self.compile_code_element(code_element.clone());
        });
        let else_body_size = self.casm_instructions.len() as i32 - instruction_number;
        // compiling else body
        body.iter().for_each(|code_element| {
            self.compile_code_element(code_element.clone());
        });
        // updating jump instruction
        self.casm_instructions[instruction_number as usize - 1] = CasmInstruction::JmpIfNeq(else_body_size+1, Operand::DerefAp(-1));
    }

    fn compile_instruction(&mut self, instr: Instruction) {
        match instr.instruction_type {
            InstructionType::Ret => self.casm_instructions.push(CasmInstruction::Ret),
            _ => todo!(),
        }
    }

    fn compile_alloc_locals(&mut self) {
        self.casm_instructions.push(CasmInstruction::IncrAp(self.size_of_locals));
    }

    pub fn compile_code_element(&mut self, code_element: CodeElement) {
        match code_element {
            CodeElement::LocalVar(ident, expr) => self.compile_local_var(ident, expr),
            CodeElement::Return(expr) => self.compile_return(expr),
            CodeElement::Function(name, args, body) => self.compile_function(name, args, body),
            CodeElement::CompoundAssertEqual(expr1, expr2) => self.compile_compound_assert_equal(expr1, expr2),
            CodeElement::If(expr, body, else_body) => self.compile_if(expr, body, else_body),
            CodeElement::Instruction(instr) => self.compile_instruction(instr),
            CodeElement::AllocLocals => self.compile_alloc_locals(),
            _ => todo!(),
        }
    }
}