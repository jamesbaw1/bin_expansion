use num_bigint::BigInt;

struct Binomial {
    a_coeff: BigInt,
    a_exp: u32,
    b_coeff: BigInt,
    b_exp: u32,
    n: u32,
}

impl Binomial {
    fn from(a_coeff: BigInt, a_exp: u32, b_coeff: BigInt, b_exp: u32, n: u32) -> Self {
        Binomial {
            a_coeff,
            a_exp,
            b_coeff,
            b_exp,
            n,
        }
    }

    fn expand(&self) -> Expression {
        let mut expr = Vec::new();
        for x in 0..=self.n {
            expr.push(Term::from(
                BigInt::from(choose(self.n, x)) * self.a_coeff.pow(self.n - x) * self.b_coeff.pow(x),
                self.a_exp * (self.n - x),
                self.b_exp * x,
            ));
        }

        Expression { expr: expr }
    }
}

impl std::fmt::Display for Binomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}a^{}{:+}b^{})^{}", self.a_coeff, self.a_exp, self.b_coeff, self.b_exp, self.n)
    }
}

struct Term {
    coeff: BigInt,
    a_exp: u32,
    b_exp: u32,
}

impl Term {
    fn from(coeff: BigInt, a_exp: u32, b_exp: u32) -> Self {
        Term {
            coeff,
            a_exp,
            b_exp,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}(a^{})(b^{})]", self.coeff, self.a_exp, self.b_exp)
    }
}

struct Expression {
    expr: Vec<Term>,
}

impl Expression {
    fn eval(&self, a: BigInt, b: BigInt) -> BigInt {
        self.expr
            .iter()
            .fold(BigInt::from(0u32), |acc, x| acc + &x.coeff * a.pow(x.a_exp) * b.pow(x.b_exp))
    }

    fn a_eval(&self, a: BigInt) -> Expression {
        Expression {
            expr: self.expr
                .iter()
                .map(|x| Term::from(&x.coeff * a.pow(x.a_exp), 0, x.b_exp))
                .collect()
        }
    }

    fn b_eval(&self, b: BigInt) -> Expression {
        Expression {
            expr: self.expr
                .iter()
                .map(|x| Term::from(&x.coeff * b.pow(x.b_exp), x.a_exp, 0))
                .collect()
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            self.expr
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

fn choose(n: u32, r: u32) -> BigInt {
    let (a, b) = if r > (n-r) {
        (r, n-r)
    } else {
        (n-r, r)
    };

    let dividend = (a+1..=n)
        .fold(BigInt::from(1u32), |acc, x| acc * x);

    let divisor = (2..=b)
        .fold(BigInt::from(1u32), |acc, x| acc * x);

    dividend/divisor
}

fn main() {
    let binomial = Binomial::from(
/*a*/   BigInt::from(1i32), 1,
/*b*/   BigInt::from(-1i32), 1,
/*n*/   10,
    );

    println!("{}", &binomial);

    println!("{}", binomial.expand());

    let start = std::time::Instant::now();
    let result = binomial
        .expand()
        .b_eval(BigInt::from(2i32));
    let duration = start.elapsed();

    println!("{}", result);

    println!("{:.2?}", duration);
}