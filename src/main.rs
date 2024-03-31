use std::fmt;

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn priority(&self) -> i32 {
        match self {
            Op::Add => 1,
            Op::Sub => 2,
            Op::Mul => 3,
            Op::Div => 4,
        }
    }

    pub fn calcualte(&self, lhs: f32, rhs: f32) -> f32 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            }
        )
    }
}

enum Val {
    Val(f32),
    BinOp(Op, Box<Expr>, Box<Expr>),
}

impl Val {
    pub fn is_val(&self) -> bool {
        match self {
            Val::Val(_) => true,
            _ => false,
        }
    }

    pub fn get_op(&self) -> Op {
        match self {
            Val::BinOp(op, _, _) => *op,
            _ => panic!("Not a binary operation"),
        }
    }

    pub fn eval(&self) -> f32 {
        let result: f32;

        match self {
            Val::Val(v) => {
                result = *v;
            }
            Val::BinOp(op, lhs, rhs) => {
                if !lhs.val.is_val() && !rhs.val.is_val() {
                    if lhs.val.get_op().priority() < rhs.val.get_op().priority() {
                        lhs.val.eval();
                    } else {
                        rhs.val.eval();
                    }
                }
                result = op.calcualte(lhs.val.eval(), rhs.val.eval());
            }
        }

        return result;
    }
}

struct Expr {
    val: Val,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.val {
                Val::Val(v) => v.to_string(),
                Val::BinOp(op, lhs, rhs) => format!("{} {} {}", lhs, op, rhs),
            }
        )
    }
}

macro_rules! num {
    ($val:tt) => {
        Expr {
            val: Val::Val($val),
        }
    };
}

macro_rules! calc {
    ($op:expr, $lhs:expr, $rhs:expr) => {
        Expr {
            val: Val::BinOp($op, Box::new($lhs), Box::new($rhs)),
        }
    };
}

fn main() {
    // 24 / 5.2 * 3 + 2.1 + 3.4 - 3
    let expr: Expr = calc!(
        Op::Add,
        calc!(Op::Sub, num!(3.4), num!(3.0)),
        calc!(
            Op::Add,
            calc!(Op::Mul, calc!(Op::Div, num!(24.0), num!(5.2)), num!(3.0)),
            num!(2.1)
        )
    );
    println!("{} = {}", expr, expr.val.eval());
}
