use num_bigint::BigInt;

#[derive(Default)]
struct Binomial {
    a_coeff: BigInt,
    a_exp: u32,
    b_coeff: BigInt,
    b_exp: u32,
    n: u32,
}

impl Binomial {
    fn define() -> Self {
        let mut binomial = Binomial::default();
    
        binomial.a_coeff = input::<BigInt>(format!(
            "({}a^{}+{}b^{})^{}\nlet \u{03B1} =",
            '\u{03B1}',
            '\u{03B2}',
            '\u{03B3}',
            '\u{03B4}',
            'n'
        ));
    
        binomial.a_exp = input::<u32>(format!(
            "\n({}a^{}+{}b^{})^{}\nlet \u{03B2} =",
            binomial.a_coeff,
            '\u{03B2}',
            '\u{03B3}',
            '\u{03B4}',
            'n'
        ));
    
        binomial.b_coeff = input::<BigInt>(format!(
            "\n({}a^{}+{}b^{})^{}\nlet \u{03B3} =",
            binomial.a_coeff,
            binomial.a_exp,
            '\u{03B3}',
            '\u{03B4}',
            'n'
        ));
    
        binomial.b_exp = input::<u32>(format!(
            "\n({}a^{}{:+}b^{})^{}\nlet \u{03B4} =",
            binomial.a_coeff,
            binomial.a_exp,
            binomial.b_coeff,
            '\u{03B4}',
            'n'
        ));
    
        binomial.n = input::<u32>(format!(
            "\n({}a^{}{:+}b^{})^{}\nlet n =",
            binomial.a_coeff,
            binomial.a_exp,
            binomial.b_coeff,
            binomial.b_exp,
            'n'
        ));
    
        println!(
            "\n({}a^{}{:+}b^{})^{}",
            binomial.a_coeff,
            binomial.a_exp,
            binomial.b_coeff,
            binomial.b_exp,
            binomial.n
        );        let start = std::time::Instant::now();
        let duration = start.elapsed();
    
        binomial
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
        write!(f, "({}a^{}{:+}b^{})^{}",
            self.a_coeff,
            self.a_exp,
            self.b_coeff,
            self.b_exp,
            self.n
        )
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

    (a+1..=n).product::<BigInt>() /
    (2..=b).product::<BigInt>()
}

fn input<T>(prompt: String) -> T
where
    T: std::str::FromStr + std::cmp::PartialEq + Default,
{
    println!("{}", prompt);
    loop {
        let mut input = String::default();
        let _ = std::io::stdin().read_line(&mut input);

        match input.trim().parse::<T>() {
            Ok(parsed_value) => {
                if parsed_value == <T>::default() { println!("WARNING: Null values can cause undefined behavior"); }
                return parsed_value;
            }
            Err(_) => {
                let type_name = std::any::type_name::<T>();
                println!("Must be {}", type_name.rsplit("::").next().unwrap_or(type_name));
                continue;
            }
        }
    }
}

fn main() {
    let binomial = Binomial::define();

    loop {
        println!("\nevaluate expression? Y/n");
        let mut inp = String::default();
        std::io::stdin().read_line(&mut inp).expect("Failed to read line");

        match inp.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                let result = binomial.expand().eval(input::<BigInt>("\nlet a =".to_string()), input::<BigInt>("\nlet b =".to_string()));
                println!("\n{}", result);
                break;
            }
            "no" | "n" => {
                let result = binomial.expand();
                println!("\n{}", result);
                break;
            }
            _ => continue,
        }
    }
}