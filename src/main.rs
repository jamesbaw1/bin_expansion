use num_bigint::BigInt;
use std::time::Instant;

#[derive(Debug)]
struct Binomial {
    a_coeff: BigInt,
    a_exp: u32,
    b_coeff: BigInt,
    b_exp: u32,
    n: u32,
}

impl Binomial {
    fn new() -> Self {
        
        Binomial {
            a_coeff: BigInt::from(1u32),
            a_exp: 1,
            b_coeff: BigInt::from(1u32),
            b_exp: 1,
            n: 1,
        }
    }

    fn from(a_coeff: BigInt, a_exp: u32, b_coeff: BigInt, b_exp: u32, n: u32) -> Self {
        Binomial {
            a_coeff,
            a_exp,
            b_coeff,
            b_exp,
            n,
        }
    }

    fn expand(&self) -> Vec<Term> {
        let mut result = Vec::new();
        for i in 0..=self.n {
            result.push(
                Term::from(
                    BigInt::from(choose(self.n, i)) * self.a_coeff.pow(self.n - i) * self.b_coeff.pow(i),
                    self.a_exp * (self.n - i),
                    self.b_exp * i,
            ));
        }

        result
    }
}

#[derive(Debug)]
struct Term {
    coeff: BigInt,
    a_exp: u32,
    b_exp: u32,
}

trait Eval {
    fn eval(&self, a: BigInt, b: BigInt) -> BigInt;
}

impl Eval for Vec<Term> {
    fn eval(&self, a: BigInt, b: BigInt) -> BigInt {
        self
            .iter()
            .fold(BigInt::from(0u32), |acc, x| acc + &x.coeff * a.pow(x.a_exp) * b.pow(x.b_exp))
    }
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

fn choose(n: u32, r: u32) -> BigInt {
    let (a, b) = if r > (n-r) {
        (r, n-r)
    } else {
        (n-r, r)
    };

    let dividend = (a+1..=n)
        .fold(BigInt::from(1u32), |acc, i| acc * i);

    let divisor = (2..=b)
        .fold(BigInt::from(1u32), |acc, i| acc * i);

    dividend/divisor
}

fn main() {
    let binomial = Binomial::from(
/*a*/   BigInt::from(-12i32), 4,
/*b*/   BigInt::from(23i32), 7,
/*n*/   15,
    );

    println!("{:?}", &binomial);

    let start = Instant::now();
    let result = binomial.expand().eval(BigInt::from(14i32), BigInt::from(20i32));
    let duration = start.elapsed();

    println!("{:?}", result);

    println!("{:.2?}", duration);
}