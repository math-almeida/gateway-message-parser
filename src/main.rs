//! Try messages: `cargo run -q -- <bytes in hex>`. Separate several messages with commas.
//! Spaces are ignored; `00x20` means the byte 00 repeated 20 times.
//! Example (a Data message with payload "hi"): cargo run -q -- 4757 02 00000002 6869
use std::panic;

fn bytes(text: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for token in text.split_whitespace() {
        let (hex, times) = match token.split_once('x') {
            Some((h, n)) => (h, n.parse::<usize>().expect("number after x")),
            None => (token, 1),
        };
        let one: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex byte"))
            .collect();
        for _ in 0..times {
            out.extend_from_slice(&one);
        }
    }
    out
}

fn main() {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    panic::set_hook(Box::new(|_| {}));
    for (i, message) in text.split(',').enumerate() {
        let input = bytes(message);
        let result = match panic::catch_unwind(|| format!("{:?}", agent_wire::parse(&input))) {
            Ok(r) if r.chars().count() > 80 => format!("{}…", r.chars().take(80).collect::<String>()),
            Ok(r) => r,
            Err(_) => "CRASH (panic)".to_string(),
        };
        println!("{}. {result}", i + 1);
    }
}
