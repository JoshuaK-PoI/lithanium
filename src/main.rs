use clap::Parser;

pub(crate) mod lang;
pub(crate) mod args;

fn main() {

    // Load file from command line argument
    let args = args::Args::parse();
    let input = std::fs::read_to_string(&args.input).unwrap_or_else(|err| {
        panic!("Error reading file: {}", err);
    });
    
    // Compile file
    let mut compiler = lang::compiler::Compiler::new(&input, &args.input);

    let ast = compiler.compile().unwrap_or_else(|err| {
        panic!("Error compiling file: {}", err);
    });

    // Print AST
    if args.debug {
        println!("{}", serde_json::to_string_pretty(&ast).unwrap());
    }
}


#[cfg(test)]
mod tests {

    use log::debug;
    use std::fs;
    use std::path::Path;
    use std::sync::Once;

    use crate::lang::compiler::Compiler;

    static INIT: Once = Once::new();

    pub(crate) fn before_each() {
        INIT.call_once(|| env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .unwrap());
    }

    #[test]
    fn test_e2e() {
        before_each();

        let test_dir = Path::new("tests/e2e");
        let test_files = fs::read_dir(test_dir).unwrap().collect::<Vec<_>>();

        assert!(&test_files.len() > &0, "No test files found");

        for file in test_files {
            let file = file.unwrap();
            let path = file.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_ext = path.extension().unwrap().to_str().unwrap();

            if file_ext == "lit" {
                debug!("Testing file: {}", file_name);
                let input = fs::read_to_string(&path).unwrap();
                let input = input.as_str();
                let mut compiler = Compiler::new(input, file_name);
                let ast = compiler.compile().unwrap_or_else(|err| {
                    panic!("Error compiling file: {}", err);
                });
                debug!("Output: {}", serde_json::to_string_pretty(&ast).unwrap());

            } else {
                debug!("Skipping file: {} (not a lithanium source file)", file_name);
            }
        }
    }
}
