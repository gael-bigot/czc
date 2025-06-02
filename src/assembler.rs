use crate::{ast::*, casm::*};
use std::{collections::HashMap};
use chumsky::error;
use json;


#[derive(Debug, Clone)]
pub struct Instruction{
    pub offdst: i32,
    pub offop0: i32,
    pub offop1: i32,
    pub imm: Option<u64>,
    pub dst: u8,
    pub op0: u8,
    pub op1: u8,
    pub res: u8,
    pub pc_update: u8,
    pub ap_update: u8,
    pub opcode: u8,
    }


impl Instruction{
    pub fn to_bytes(&self) -> (u64, Option<u64>) {
        let mut res: u64 = 0;
        res += (self.offdst + 0x8000) as u64;
        res += ((self.offop0 + 0x8000) as u64) << 16;
        res += ((self.offop1 + 0x8000) as u64) << 32;
        res += (self.dst as u64) << 48;
        res += (self.op0 as u64) << 49;
        res += (self.op1 as u64) << 50;
        res += (self.res as u64) << 53;
        res += (self.pc_update as u64) << 55;
        res += (self.ap_update as u64) << 58;
        res += (self.opcode as u64) << 60;
        match self.imm {
            Some(imm) => (res, Some(imm)),
            None => (res, None),
        }
    }
}



pub fn build_instruction(instruction: CasmInstruction) -> Instruction {
    match instruction {
        CasmInstruction::CallRel(offset) => {
            let offdst = 0;
            let offop0 = 1;
            let offop1 = 1;
            let imm = if offset < 0 {0x7FFFFFFF + offset} else {offset};
            let imm = Some(imm as u64);
            let dst = 0;
            let op0 = 0;
            let op1 = 1;
            let res = 0;
            let pc_update = 2;
            let ap_update = 0;
            let opcode = 1;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::CallAbs(address) => {
            let offdst = 0;
            let offop0 = 1;
            let offop1 = 1;
            let imm = Some(address as u64);
            let dst = 0;
            let op0 = 0;
            let op1 = 1;
            let res = 0;
            let pc_update = 1;
            let ap_update = 0;
            let opcode = 1;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::Set { left, op, incr_ap } => {
            let offdst = match left {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                _ => unreachable!(),
            };
            let offop0 = -1;
            let offop1 = match op {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                Operand::Int(n) => 1,
            };
            let imm = match op {
                Operand::Int(n) => Some(n),
                _ => None,
            };
            let dst = match left {
                Operand::DerefFp(offset) => 1,
                Operand::DerefAp(offset) => 0,
                _ => unreachable!(),
            };
            let op0 = 1;
            let op1 = match op {
                Operand::DerefFp(offset) => 2,
                Operand::DerefAp(offset) => 4,
                Operand::Int(n) => 1,
            };
            let res = 0;
            let pc_update = 0;
            let ap_update = if incr_ap { 2 } else { 0 };
            let opcode = 4;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::Add { left, op1, op2 } => {
            let offdst = match left {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                _ => unreachable!(),
            };
            let offop0 = match op1 {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                Operand::Int(n) => unreachable!(),
            };
            let offop1 = match op2 {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                Operand::Int(n) => 1,
            };
            let imm = match op2 {
                Operand::Int(n) => Some(n),
                _ => None,
            };
            let dst = match left {
                Operand::DerefFp(offset) => 1,
                Operand::DerefAp(offset) => 0,
                _ => unreachable!(),
            };
            let op0 = match op1 {
                Operand::DerefFp(offset) => 1,
                Operand::DerefAp(offset) => 0,
                Operand::Int(n) => unreachable!(),
            };
            let op1 = match op2 {
                Operand::DerefFp(offset) => 2,
                Operand::DerefAp(offset) => 4,
                Operand::Int(n) => 1,
            };
            let res = 1;
            let pc_update = 0;
            let ap_update = 2;
            let opcode = 4;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::Mul { left, op1, op2 } => {
            let offdst = match left {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                _ => unreachable!(),
            };
            let offop0 = match op1 {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                Operand::Int(n) => 1,
            };
            let offop1 = match op2 {
                Operand::DerefFp(offset) => offset,
                Operand::DerefAp(offset) => offset,
                Operand::Int(n) => 1,
            };
            let imm = match op2 {
                Operand::Int(n) => Some(n),
                _ => None,
            };
            let dst = match left {
                Operand::DerefFp(offset) => 1,
                Operand::DerefAp(offset) => 0,
                _ => unreachable!(),
            };
            let op0 = match op1 {
                Operand::DerefFp(offset) => 1,
                Operand::DerefAp(offset) => 0,
                Operand::Int(n) => unreachable!(),
            };
            let op1 = match op2 {
                Operand::DerefFp(offset) => 2,
                Operand::DerefAp(offset) => 4,
                Operand::Int(n) => 1,
            };
            let res = 2;
            let pc_update = 0;
            let ap_update = 2;
            let opcode = 4;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::Ret => {
            let offdst = -2;
            let offop0 = -1;
            let offop1 = -1;
            let imm = None;
            let dst = 1;
            let op0 = 1;
            let op1 = 2;
            let res = 0;
            let pc_update = 1;
            let ap_update = 0;
            let opcode = 2;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        CasmInstruction::IncrAp(n) => {
            let offdst = -1;
            let offop0 = -1;
            let offop1 = 1;
            let imm = Some(n);
            let dst = 1;
            let op0 = 1;
            let op1 = 1;
            let res = 0;
            let pc_update = 0;
            let ap_update = 1;
            let opcode = 0;
            Instruction{offdst, offop0, offop1, imm, dst, op0, op1, res, pc_update, ap_update, opcode}
        }
        _ => todo!(),
    }
}


fn nops(i: CasmInstruction) -> u64{
    let bytecode = build_instruction(i);
    if let Some(imm) = bytecode.imm {
        return 2;
    }
    return 1;
}

pub struct Assembler{
    pub casm: Vec<CasmInstruction>,
    pub instructions: Vec<Instruction>,
    pub function_adresses: HashMap<String, u64>,
}

impl Assembler{
    pub fn new() -> Self {
        Self { casm: Vec::new(), instructions: Vec::new(), function_adresses: HashMap::new() }
    }


    pub fn resolve_calls(&mut self) {
        let mut new = Vec::new();
        let mut instruction_number = 0;
        //let mut function_adresses = HashMap::new();
        for instruction in self.casm.clone() {
            match instruction {
                CasmInstruction::Label(label) => {
                    self.function_adresses.insert(label, instruction_number);
                }
                CasmInstruction::Call(label) => {
                    instruction_number += 2;
                }
                _ => {
                    instruction_number += nops(instruction.clone());
                }
            }
        }
        instruction_number = 0;
        for instruction in self.casm.clone() {
            match instruction {
                CasmInstruction::Call(label) => {
                    new.push(CasmInstruction::CallRel(self.function_adresses[&label] as i32  - instruction_number as i32));
                    instruction_number += 2;
                }
                CasmInstruction::Label(label) => {}
                _ => {
                    new.push(instruction.clone());
                    instruction_number += nops(instruction);
                }
            }
        }
        self.casm = new;
    }

    pub fn build_instructions(&mut self) {
        for instruction in self.casm.clone() {
            self.instructions.push(build_instruction(instruction));
        }
    }

    pub fn to_json(&self) -> String {
        let mut data = json::JsonValue::new_object();
        data["attributes"] = json::JsonValue::new_array();
        data["builtins"] = json::JsonValue::new_array();
        data["compiler_version"] = json::JsonValue::from("0.1");
        data["data"] = json::JsonValue::new_array();
        for instruction in self.instructions.clone() {
            let (bytes, imm) = instruction.to_bytes();
            data["data"].push(format!("{:#x}", bytes));
            if let Some(imm) = imm {
                data["data"].push(format!("{:#x}", imm));
            }
        }
        data["hints"] = json::JsonValue::new_object();
        data["identifiers"] = json::JsonValue::new_object();
        for (label, address) in self.function_adresses.clone() {
            let label2 = format!("__main__.{}", label);
            data["identifiers"][label2.clone()] = json::JsonValue::new_object();
            data["identifiers"][label2.clone()]["decorators"] = json::JsonValue::new_array();
            data["identifiers"][label2.clone()]["pc"] = json::JsonValue::from(address);
            data["identifiers"][label2.clone()]["type"] = json::JsonValue::from("function");
        }
        data["main_scope"] = json::JsonValue::from("__main__");
        data["prime"] = json::JsonValue::from("0x7fffffff");
        data["reference_manager"] = json::JsonValue::new_object();
        data["reference_manager"]["references"] = json::JsonValue::new_array();
        data.to_string()
    }
}