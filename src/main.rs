pub(crate) mod lang;
    
fn main() {
    println!("Hello, world!");
    
}


#[cfg(test)]
mod tests {

    use log::debug;
    use std::fs;
    use std::path::Path;

    use crate::lang::compiler::Compiler;

    pub(crate) fn before_each() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init()
            .unwrap();
        // Logger should be set to debug level for tests
        debug!("Logger initialized");
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
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let file_ext = path.extension().unwrap().to_str().unwrap();

            if file_ext == "lit" {
                debug!("Testing file: {}", file_name);
                let input = fs::read_to_string(&path).unwrap();
                let input = input.as_str();
                let mut compiler = Compiler::new(input, file_name);
                let ast = compiler.compile().unwrap_or_else(|err| {
                    panic!("Error compiling file: {}", err);
                });
                let output = format!("{:?}", ast);
                debug!("Output: {}", output);

            } else {
                debug!("Skipping file: {}", file_name);
            }
        }
    }
}
