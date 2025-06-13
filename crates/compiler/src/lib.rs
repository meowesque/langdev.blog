pub mod error;
pub mod options;
pub mod prelude;

use options::Options;
use prelude::*;
use std::{default, io::Write, path::Path};

pub fn compile(options: &Options, archive: impl AsRef<Path>) -> Result<()> {
  let file = std::fs::File::open(archive.as_ref())?;
  let mut archive = zip::ZipArchive::new(file)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;

    let subpath = match file.enclosed_name() {
      Some(path) => path,
      None => continue,
    };

    let fullpath = options.output.join(subpath);

    if let Some(parent) = fullpath.parent() {
      std::fs::create_dir_all(&parent)?;
    }

    match () {
      _ if file.is_dir() => std::fs::create_dir_all(&fullpath)?,
      _ if file.is_file() => {
        match fullpath.extension() {
          // Markdown
          Some(ext) if ext == "md" => {
            let mut outfile = std::fs::File::create(&fullpath.with_extension("html"))?;
            let content = std::io::read_to_string(file)?;

            let result = markdown::to_html_with_options(
              &content,
              &markdown::Options {
                parse: markdown::ParseOptions::gfm(),
                compile: markdown::CompileOptions::gfm(),
              },
            );

            match result {
              Ok(html) => {
                outfile.write_all(html.as_bytes())?;
              }
              Err(_message) => {
                todo!("Not handled! (:")
              }
            }
          }
          // Etc.
          _ => {
            let mut outfile = std::fs::File::create(&fullpath)?;

            std::io::copy(&mut file, &mut outfile)?;
          }
        }
      }
      _ if file.is_symlink() => todo!("Symlinks arent implemented yet"),
      _ => unreachable!(),
    }
  }

  Ok(())
}
