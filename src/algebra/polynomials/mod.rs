pub mod operators;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
/// Polynomials
pub struct Pol(Vec<f64>);

impl Pol {
    /// Create polynomials form vector
    pub fn new(input: Vec<f64>) -> Pol {
        if input.is_empty() {
            return Pol(vec![0.]);
        }
        Pol(input)
    }

    pub fn solve(&self) -> Vec<f64> {
        if self.0.len() == 3 {
            let a = self.0[2];
            let b = self.0[1];
            let c = self.0[0];

            vec![
                (-b + (b.powf(2.) - 4. * a * c).sqrt()) / (2. * a),
                (-b - (b.powf(2.) - 4. * a * c).sqrt()) / (2. * a),
            ]
        } else if self.0.len() == 4 && self.0[0] == 0. {
            let a = self.0[3];
            let b = self.0[2];
            let c = self.0[1];

            vec![
                0.,
                (-b + (b.powf(2.) - 4. * a * c).sqrt()) / (2. * a),
                (-b - (b.powf(2.) - 4. * a * c).sqrt()) / (2. * a),
            ]
        } else {
            panic!("Can't solve polynomial")
        }
    }
}

impl Display for Pol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (i, el) in self.0.iter().enumerate() {
            if *el != 0. {
                let el_str = (el.abs()).to_string();
                let i_str = i.to_string();

                result += &format!(
                    "{} {}{}{}{} ",
                    if *el > 0. { "+" } else { "-" },
                    if *el != 1. && *el != -1. { &el_str } else { "" },
                    if i != 0 { "x" } else { "" },
                    if i != 0 && i != 1 { "^" } else { "" },
                    if i != 1 { &i_str } else { "" }
                );
            }
        }

        write!(f, "{}", result)
    }
}

