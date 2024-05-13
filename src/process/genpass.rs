use std::fmt::Display;

use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;
const NUMBER: &[u8] = b"0123456789";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const SYMBOL: &[u8] = b"~!@#$%^&*()-_=+[{]}";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
    count: u32,
    output_strength: bool,
) -> anyhow::Result<()> {
    let mut chars = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let mut password = Vec::with_capacity(length as usize);
        if upper {
            chars.extend(UPPER);
            password.push(*UPPER.choose(&mut rng).expect("UPPER is empty"));
        }
        if lower {
            chars.extend(LOWER);
            password.push(*LOWER.choose(&mut rng).expect("LOWER is empty"));
        }
        if number {
            chars.extend(NUMBER);
            password.push(*NUMBER.choose(&mut rng).expect("NUMBER is empty"));
        }
        if symbol {
            chars.extend(SYMBOL);
            password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL is empty"));
        }
        if length < password.len() as u8 {
            return Err(anyhow::anyhow!(
                "length is too short to generate a password"
            ));
        }
        for _ in 0..length - (password.len() as u8) {
            password.push(*chars.choose(&mut rng).expect("chars is empty"));
        }
        password.shuffle(&mut rng);
        let pwd = String::from_utf8(password)?;
        if output_strength {
            let estimate = zxcvbn(&pwd, &[])?;
            println!(
                "{} the password strength is {}",
                pwd,
                PwdScore::from(estimate.score())
            );
        } else {
            println!("{}", pwd);
        }
    }
    Ok(())
}

enum PwdScore {
    Weak,
    Medium,
    Strong,
}

impl From<u8> for PwdScore {
    fn from(score: u8) -> Self {
        match score {
            0..=2 => PwdScore::Weak,
            3 => PwdScore::Medium,
            4.. => PwdScore::Strong,
        }
    }
}

impl Display for PwdScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PwdScore::Weak => write!(f, "weak"),
            PwdScore::Medium => write!(f, "medium"),
            PwdScore::Strong => write!(f, "strong"),
        }
    }
}
