use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

fn unary_op() -> Vec<(u8,u8)> {
    (0u8..=255).map(|x| (x,0)).collect()
}

fn binary_op() -> Vec<(u8,u8)> {
    (0u8..=255).flat_map(|x| (0u8..=255).map(move |y| (x,y))).collect()
}

fn op_gen(opcode: u8, operands: Vec<(u8,u8)>, op: impl Fn(u8, u8) -> u8) {
    for (x_in, y) in operands {
        let sum = op(x_in, y);
        println!("{opcode:03b} {x_in:08b} {y:08b} {sum:08b}")
    }
}

fn main() {
    println!("OP[3] X_OUT[8] Y[8] X_IN[8]");
    op_gen(0b000, binary_op(), |a,b| a.overflowing_add(b).0);
    op_gen(0b001, binary_op(), |a,b| a.overflowing_sub(b).0);
    op_gen(0b100, binary_op(), |a,b| a.bitxor(b));
    op_gen(0b110, binary_op(), |a,b| a.bitand(b));
    op_gen(0b111, binary_op(), |a,b| a.bitor(b));
    op_gen(0b011, unary_op(), |a,_| a.shl(1));
    op_gen(0b010, unary_op(), |a,_| a.shr(1));
    op_gen(0b101, unary_op(), |a,_| a.not());
}
