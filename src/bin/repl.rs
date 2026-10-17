#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use cfg_if::cfg_if;

use Nex::Compile;
cfg_if! {
    if #[cfg(feature = "jit")] {
        use Nex::Jit as Engine;
    }
    else if #[cfg(feature = "interpreter")] {
        use Nex::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use Nex::vm::bytecode::Interpreter as Engine;
        use Nex::VM;
    }
}

// ANCHOR: repl
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Nex prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                cfg_if! {
                    if #[cfg(any(feature = "jit", feature = "interpreter"))] {
                        match Engine::from_source(line) {
                            Ok(result) => println!("{}", result),
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    else if #[cfg(feature = "vm")] {
                        let byte_code = Engine::from_source(line);
                        println!("byte code: {:?}", byte_code);
                        let mut vm = VM::new(byte_code);
                        vm.run();
                        println!("{}", vm.pop_last());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
    // ANCHOR_END: repl
}
