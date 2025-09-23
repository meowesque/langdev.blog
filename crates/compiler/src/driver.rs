fn main() -> Result<(), compiler::prelude::Error> {
  let details = compiler::compile(
    &compiler::options::Options {
      output: std::path::Path::new("./test/output"),
      trim_rootdir: true,
      garnish_html: true,
      minify_html: true,
      minify_css: true,
    },
    "./test/lorem-ipsum.zip",
  )?;

  dbg!(details);

  Ok(())
}
