pub mod error;
pub mod meta;
pub mod options;
pub mod prelude;

use options::Options;
use prelude::*;
use std::{ffi::OsStr, io::Write, path::Path};

use crate::meta::Meta;

pub fn compile(options: &Options, archive: impl AsRef<Path>) -> Result<Meta> {
  let mut meta: Option<Meta> = None;

  let file = std::fs::File::open(archive.as_ref())?;
  let mut archive = zip::ZipArchive::new(file)?;

  let root_dir = archive.root_dir(zip::read::root_dir_common_filter)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;

    let subpath = match file.enclosed_name() {
      Some(path) => path,
      None => continue,
    };

    let fullpath = options.output.join(match root_dir {
      Some(ref root_dir) if options.trim_rootdir => subpath
        .strip_prefix(root_dir)
        .map(|x| x.to_path_buf())
        .unwrap_or(subpath),
      _ => subpath,
    });

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

            let html = markdown::to_html_with_options(
              &content,
              &markdown::Options {
                parse: markdown::ParseOptions::gfm(),
                compile: markdown::CompileOptions::gfm(),
              },
            )
            .map_err(Error::MarkdownError)?;

            outfile.write_all(html.as_bytes())?;
          }
          // Post metadata file
          _ if fullpath.file_name() == Some(OsStr::new("meta.toml")) => {
            let content = std::io::read_to_string(file)?;
            meta = Some(toml::from_str(&content)?);
          }
          // Index html
          _ if fullpath.file_name() == Some(OsStr::new("index.html")) => {
            todo!("Not yet implemented")
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

  meta.ok_or(Error::MissingMetaFile)
}
