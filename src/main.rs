// use num_bigint::BigUint;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
struct Term {
    coefficient: i32,
    symbol: char,
    exponent: u32,
}

impl Term {
    fn new(symbol: char) -> Self {        
        Term {
            coefficient: 1,
            symbol,
            exponent: 1, 
        }
    }

    fn from(coefficient: i32, symbol: char, exponent: u32) -> Self {        
        Term {
            coefficient,
            symbol,
            exponent, 
        }
    }

    fn eval(&self, value: i32) -> i32 {
        self.coefficient * value.pow(self.exponent)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}^{}",
            if self.coefficient != 1 {
                format!("{}", self.coefficient)
            } else {
                String::new()
            },
            self.symbol,
            self.exponent
        )
    }
}

fn expand(a: Term, b: Term, n: u32) -> Vec<(i32, Term, Term)> {
    let mut binomial = Vec::new();
    for x in 0..=n {
        let y = n-x;
        let c = choose(n, x);
        binomial.push(
            (
                c*a.coefficient.pow(y)*b.coefficient.pow(x),
                Term::from(1, 'a', a.exponent*y),
                Term::from(1, 'b', b.exponent*x)
            )
        );
    }

    binomial
}

fn substitute(binomial: Vec<(i32, Term, Term)>, a: i32, b: i32) -> i32 {
    binomial
    .iter()
    .fold(0, |acc, x| acc + x.0 * x.1.eval(a) * x.2.eval(b))
}

fn choose(n: u32, r: u32) -> i32 {
    let (a, b) = if r > (n-r) {
        (r, n-r)
    } else {
        (n-r, r)
    };

    let dividend = (a+1..=n)
        .fold(1, |acc, i| acc * i);

    let divisor = (2..=b)
        .fold(1, |acc, i| acc * i);

    (dividend/divisor) as i32
}


fn main() {
    // let start = Instant::now();
    // let result = pascal(5);
    // let duration = start.elapsed();
    // println!("{:.2?}\n{:?}", duration, result);

    let a = Term::from(2, 'a', 3);
    let b = Term::new('b');
    let n = 3;

    println!("({} + {})^{}", &a, &b, &n);

    let start = Instant::now();
    let result = expand(a, b, n);
    let duration = start.elapsed();

    println!("{:.2?}", duration);

    println!("{}", substitute(result, 2, 12))
}