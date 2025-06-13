fn main() -> Result<(), compiler::prelude::Error> {
  compiler::compile(
    &compiler::options::Options {
      output: std::path::Path::new("./test/output"),
    },
    "./test/lorem-ipsum.zip",
  )
}
