use cfg_if::cfg_if;

use Nex::Compile;
cfg_if! {
    if #[cfg(feature = "jit")] {
        use Nex::Jit as Engine;
    }
    else if #[cfg(feature = "vm")] {
        use Nex::VM as Engine;
    }
    else {
        use Nex::Interpreter as Engine;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }
    println!(
        "{:?}",
        Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap()).unwrap()
    );
}
