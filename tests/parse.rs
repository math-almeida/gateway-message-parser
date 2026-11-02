use agent_wire::{parse, Message};

/// Builds a message: magic "GW", type, payload length (u32, big-endian), payload.
pub fn frame(msg_type: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = b"GW".to_vec();
    out.push(msg_type);
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

/// Builds a Hello payload: session id (u64), keepalive in seconds (u32).
pub fn hello(session_id: u64, keepalive_secs: u32) -> Vec<u8> {
    let mut p = session_id.to_be_bytes().to_vec();
    p.extend_from_slice(&keepalive_secs.to_be_bytes());
    p
}

#[test]
fn parses_hello() {
    let msg = parse(&frame(1, &hello(42, 30))).unwrap();
    assert_eq!(msg, Message::Hello { session_id: 42, keepalive_secs: 30 });
}

#[test]
fn parses_data() {
    assert_eq!(parse(&frame(2, b"abc")).unwrap(), Message::Data(b"abc".to_vec()));
}

// To check that a message is rejected, write a test like this one:
//
// #[test]
// fn rejects_something() {
//     assert!(parse(&frame(2, b"...")).is_err());
// }
