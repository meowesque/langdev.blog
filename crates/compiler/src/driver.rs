fn main() -> Result<(), compiler::prelude::Error> {
  let meta = compiler::compile(
    &compiler::options::Options {
      output: std::path::Path::new("./test/output"),
      trim_rootdir: true,
    },
    "./test/lorem-ipsum.zip",
  )?;

  dbg!(meta);

  Ok(())
}
