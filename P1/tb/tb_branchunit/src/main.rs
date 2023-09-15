fn op_gen(jmp: bool, jgt: bool, jz: bool, op: impl Fn(i8) -> bool) {
    for x_out in 0u8..=255 {
        println!("{} {} {} {:08b} {}", jmp as u8, jgt as u8, jz as u8, x_out, (op(x_out as i8) as u8))
    }
}

fn main() {
    println!("JMP JGT JZ X_OUT[8] BRANCH_TAKEN");
    op_gen(true, false, false, |_| true);
    op_gen(false, true, false, |a| a>=0);
    op_gen(false, false, true, |a| a==0);
    op_gen(false, false, false, |_| false);
}
