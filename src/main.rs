#[allow(dead_code)]

mod runtime;

use crate::runtime::{CirylRuntime, RuntimeReturn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = CirylRuntime::new();

    loop {
        match rt.task()? {
            RuntimeReturn::Exit => break,
            RuntimeReturn::Continue => {}
        };
    }

    Ok(())
}
