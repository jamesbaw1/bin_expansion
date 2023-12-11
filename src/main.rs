use num_bigint::BigInt;
use std::time::Instant;
// use std::fmt;

#[derive(Debug)]
struct Term {
    coeff: BigInt,
    a_exp: u32,
    b_exp: u32,
}

trait Eval {
    fn eval(&self, a: BigInt, b: BigInt) -> BigInt;
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

impl Eval for Vec<Term> {
    fn eval(&self, a: BigInt, b: BigInt) -> BigInt {
        self
            .iter()
            .fold(BigInt::from(0u32), |acc, x| acc + &x.coeff * a.pow(x.a_exp) * b.pow(x.b_exp))
    }
}

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

fn choose(n: u32, r: u32) -> u32 {
    let (a, b) = if r > (n-r) {
        (r, n-r)
    } else {
        (n-r, r)
    };

    let dividend = (a+1..=n)
        .fold(1, |acc, i| acc * i);

    let divisor = (2..=b)
        .fold(1, |acc, i| acc * i);

    dividend/divisor
}

fn main() {
    let a_coeff = BigInt::from(-12i32);
    let a_exp = 4;
    let b_coeff = BigInt::from(23i32);
    let b_exp = 7;
    let n = 15;

    let binomial = Binomial::from(a_coeff, a_exp, b_coeff, b_exp, n);

    println!("{:?}", &binomial);

    let start = Instant::now();
    let result = binomial.expand();
    let duration = start.elapsed();

    let r = result.eval(BigInt::from(2i32), BigInt::from(2i32));

    println!("{:?}", r);

    println!("{:.2?}", duration);
}