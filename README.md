# Code exercise: reading a network message (Rust)

`src/lib.rs` has one function, `parse`, that reads a message received from the
network. The message format is in the comment at the top of that file.

The function has problems: some inputs make it crash, and others give a wrong
result without any error.

## Your task (about 20 minutes)

1. **Points 1 to 6** in `src/lib.rs`. Each point has 3 lines, marked A, B and C.
   Line A is active; B and C are turned off with `//`. Keep the best line and
   delete the other two. To choose B or C, delete its `//` and delete line A.
   At one of the points, line A is already the right one.
2. **One more problem** has no number. Find it and fix it by writing the code
   yourself.
3. When you finish, tell the interviewer. You will be asked to explain some of
   your choices in one sentence.

Think out loud while you work.

## Trying messages

Type the message bytes in hex. This example is a Data message whose payload is "hi":

```bash
cargo run -q -- 4757 02 00000002 6869
#               GW   │  length 2  "hi"
#                    type 2 = Data
```

It prints what `parse` returns, or `CRASH`. You can send several messages
separated by commas. `00x20` means the byte 00 repeated 20 times. Use it
instead of writing tests. `cargo test` runs the existing tests (2 should pass).

## Rules

No AI tools: no AI chat, no coding agents, no AI code completion. The Rust docs
and a regular web search are fine. Standard library only.

## Setup

Latest stable Rust (1.85 or newer). Without Rust:
`docker run --rm -v "$PWD":/w -w /w rust:1.99.0-slim cargo test`
