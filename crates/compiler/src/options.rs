use std::path::Path;

pub struct Options<'a> {
  pub trim_rootdir: bool,
  pub output: &'a Path,
}
