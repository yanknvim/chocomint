use crate::parser::{Op, Tree};
use std::collections::HashMap;

pub struct Generator {
    trees: Vec<Tree>,
    env: Vec<HashMap<String, i64>>,
    stack_offset: i64,
}

impl Generator {
    pub fn new(trees: Vec<Tree>) -> Self {
        Self {
            trees,
            env: Vec::new(),
            stack_offset: 0,
        }
    }

    pub fn declare(&mut self, name: String) -> i64 {
        self.stack_offset -= 8;
        let offset = self.stack_offset;
        self.env.last_mut().unwrap().insert(name, offset);
        offset
    }

    pub fn lookup(&self, name: &str) -> Option<i64> {
        for scope in self.env.iter().rev() {
            if let Some(&offset) = scope.get(name) {
                return Some(offset);
            }
        }

        None
    }

    pub fn generate(&mut self) {
        self.env.push(HashMap::new());
        for tree in self.trees.clone() {
            self.generate_tree(tree);
            self.pop("t0");
        }
        self.env.pop();
    }

    pub fn generate_tree(&mut self, tree: Tree) {
        match tree {
            Tree::Integer(n) => {
                println!("  li t0, {}", n);
                self.push("t0");
            }
            Tree::BinOp(op, lhs, rhs) => {
                self.generate_tree(*lhs);
                self.generate_tree(*rhs);

                self.pop("t1");
                self.pop("t0");

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

                self.push("t0");
            }
            Tree::Assign(lhs, rhs) => {
                if let Tree::Var(name) = *lhs {
                    self.generate_tree(*rhs);
                    self.pop("t0");

                    let offset = match self.lookup(name.as_str()) {
                        Some(offset) => offset,
                        None => self.declare(name),
                    };

                    println!("  sd t0, {}(fp)", offset);
                    self.push("t0");
                } else {
                    panic!("unexpected left hand");
                }
            }
            Tree::Var(name) => {
                let offset = self.lookup(&name).expect("not declared");
                println!("  ld t0, {}(fp)", offset);
                self.push("t0");
            }
        }
    }

    pub fn push(&self, reg: &str) {
        println!("  addi sp, sp, -8");
        println!("  sd {}, 0(sp)", reg);
    }

    pub fn pop(&self, reg: &str) {
        println!("  ld {}, 0(sp)", reg);
        println!("  addi sp, sp, 8");
    }
}
