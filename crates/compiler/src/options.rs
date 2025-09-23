use std::path::Path;

pub struct Options<'a> {
  pub trim_rootdir: bool,
  pub output: &'a Path,
  pub garnish_html: bool,
  pub minify_html: bool,
  pub minify_css: bool,
}
