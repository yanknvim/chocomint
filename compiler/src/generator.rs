use crate::parser::{Op, Tree};

pub fn push(reg: &str) {
    println!("  addi sp, sp, -8");
    println!("  sd {}, 0(sp)", reg);
}

pub fn pop(reg: &str) {
    println!("  ld {}, 0(sp)", reg);
    println!("  addi sp, sp, 8");
}

pub fn generate(tree: Tree) {
    match tree {
        Tree::Integer(n) => {
            println!("  li t0, {}", n);
            push("t0");
        }
        Tree::BinOp(op, lhs, rhs) => {
            generate(*lhs);
            generate(*rhs);

            pop("t1");
            pop("t0");

            match op {
                Op::Add => println!("  add t0, t0, t1"),
                Op::Sub => println!("  sub t0, t0, t1"),
                Op::Mul => println!("  mul t0, t0, t1"),
                Op::Div => println!("  div t0, t0, t1"),

                Op::Eq => {
                    println!("  sub t0, t0, t1");
                    println!("  seqz t0, t0");
                }
                Op::NotEq => {
                    println!("  sub t0, t0, t1");
                    println!("  snez t0, t0");
                }
                Op::GreaterThan => {
                    println!("  slt t0, t1, t0");
                }
                Op::LessThan => {
                    println!("  slt t0, t0, t1");
                }
                Op::GreaterThanOrEq => {
                    println!("  slt t0, t0, t1");
                    println!("  xori t0, t0, 1");
                }
                Op::LessThanOrEq => {
                    println!("  slt t0, t1, t0");
                    println!("  xori t0, t0, 1");
                }
            }

            push("t0");
        }
    }
}
