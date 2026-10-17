use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use rand::Rng;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

/// 易混字符：0 O o I l 1
const AMBIGUOUS: &[char] = &['0', 'O', 'o', 'I', 'l', '1'];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Password,
    Username,
    Nickname,
}

#[derive(Clone, Debug)]
pub struct PasswordOptions {
    pub length: u32,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 20,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
            exclude_ambiguous: true,
        }
    }
}

pub struct Strength {
    pub label: &'static str,
    pub bits: f64,
    /// 0.0 ~ 1.0，用于进度条
    pub ratio: f32,
}

pub fn password_charset(opts: &PasswordOptions) -> String {
    let mut pool = String::new();
    if opts.lowercase {
        pool.push_str(&filter_ambiguous(LOWER, opts.exclude_ambiguous));
    }
    if opts.uppercase {
        pool.push_str(&filter_ambiguous(UPPER, opts.exclude_ambiguous));
    }
    if opts.digits {
        pool.push_str(&filter_ambiguous(DIGITS, opts.exclude_ambiguous));
    }
    if opts.symbols {
        pool.push_str(SYMBOLS);
    }
    pool
}

fn filter_ambiguous(s: &str, exclude: bool) -> String {
    if !exclude {
        return s.to_string();
    }
    s.chars()
        .filter(|c| !AMBIGUOUS.contains(c))
        .collect()
}

pub fn password_strength(opts: &PasswordOptions) -> Strength {
    let charset = password_charset(opts);
    let n = charset.len();
    if n == 0 || opts.length == 0 {
        return Strength {
            label: "—",
            bits: 0.0,
            ratio: 0.0,
        };
    }
    let bits = (opts.length as f64) * (n as f64).log2();
    let ratio = (bits / 128.0).min(1.0) as f32;
    let label = if bits < 40.0 {
        "很弱"
    } else if bits < 60.0 {
        "弱"
    } else if bits < 80.0 {
        "中等"
    } else if bits < 100.0 {
        "强"
    } else {
        "很强"
    };
    Strength {
        label,
        bits,
        ratio,
    }
}

pub fn generate_password(opts: &PasswordOptions) -> String {
    let charset = password_charset(opts);
    let chars: Vec<char> = charset.chars().collect();
    if chars.is_empty() || opts.length == 0 {
        return String::new();
    }
    let mut rng = OsRng;
    (0..opts.length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

pub fn generate_passwords(opts: &PasswordOptions, count: usize) -> String {
    (0..count)
        .map(|_| generate_password(opts))
        .collect::<Vec<_>>()
        .join("\n")
}

/// 用户名：小写字母 + 数字，可含下划线，首字符为字母。
pub fn generate_username(length: u32) -> String {
    let len = length.clamp(4, 32) as usize;
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let body: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789_".chars().collect();
    let mut rng = OsRng;
    let mut s = vec![letters[rng.gen_range(0..letters.len())]];
    while s.len() < len {
        s.push(body[rng.gen_range(0..body.len())]);
    }
    s.into_iter().collect()
}

const NICK_ADJ: [&str; 16] = [
    "静默",
    "浅夏",
    "远山",
    "微光",
    "逐风",
    "清欢",
    "星落",
    "云深",
    "听雨",
    "拾穗",
    "半盏",
    "南巷",
    "北鸢",
    "青衫",
    "白茶",
    "晚钟",
];

const NICK_NOUN: [&str; 16] = [
    "旅人",
    "行者",
    "墨客",
    "观星者",
    "拾梦人",
    "渡舟",
    "栖鹤",
    "听潮",
    "煮茶",
    "折枝",
    "问月",
    "寻鹿",
    "枕书",
    "候风",
    "写序",
    "归人",
];

pub fn generate_nickname() -> String {
    let mut rng = OsRng;
    let adj = NICK_ADJ.choose(&mut rng).copied().unwrap_or("随密");
    let noun = NICK_NOUN.choose(&mut rng).copied().unwrap_or("旅人");
    if rng.gen_bool(0.35) {
        format!("{}{}{}", adj, noun, rng.gen_range(10..100))
    } else {
        format!("{}{}", adj, noun)
    }
}
