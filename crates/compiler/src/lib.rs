pub mod error;
pub mod meta;
pub mod options;
pub mod prelude;

use options::Options;
use prelude::*;
use std::{ffi::OsStr, io::Write, path::Path};

use crate::meta::Meta;

#[derive(Debug)]
pub struct CompilationDetails {
  pub meta: Meta,
}

pub fn compile(options: &Options, archive: impl AsRef<Path>) -> Result<CompilationDetails> {
  // TODO(meowesque): This function is getting a bit long, maybe split it up? (:

  let mut markdown_content: Option<String> = None;
  let mut css_content: Option<String> = None;
  let mut meta: Option<Meta> = None;

  let file = std::fs::File::open(archive.as_ref())?;
  let mut zip = zip::ZipArchive::new(file)?;

  let root_dir = zip.root_dir(zip::read::root_dir_common_filter)?;

  for i in 0..zip.len() {
    let mut file = zip.by_index(i)?;

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
          _ if fullpath.file_name() == Some(OsStr::new("post.md")) => {
            markdown_content = Some(std::io::read_to_string(&mut file)?);
          }
          // Blog post metadata
          _ if fullpath.file_name() == Some(OsStr::new("meta.toml")) => {
            let content = std::io::read_to_string(file)?;
            meta = Some(toml::from_str(&content)?);
          }
          // CSS
          _ if fullpath.file_name() == Some(OsStr::new("styles.css")) => {
            // TODO(meowesque): Validate CSS
            // TODO(meowesque): Minify CSS
            css_content = Some(std::io::read_to_string(&mut file)?);
          }
          // Etc.
          _ => {
            let mut outfile = std::fs::File::create(&fullpath)?;
            std::io::copy(&mut file, &mut outfile)?;
          }
          _ => return Err(Error::ForbiddenFile("Unknown file type")),
        }
      }
      // _ if file.is_symlink() => todo!("Symlinks arent implemented yet"),
      _ => {}
    }
  }

  let Some(markdown_content) = markdown_content else {
    return Err(Error::MissingPostFile);
  };

  let Some(meta) = meta else {
    return Err(Error::MissingMetaFile);
  };

  let markdown_html = markdown::to_html_with_options(
    &markdown_content,
    &markdown::Options {
      parse: markdown::ParseOptions::gfm(),
      compile: markdown::CompileOptions::gfm(),
    },
  )
  .map_err(Error::MarkdownError)?;

  if options.garnish_html {
    let css = css_content.unwrap_or_default();

    let template = include_str!("html/basic.html");
    let garnished_html = template
      .replace("{{title}}", &meta.title)
      .replace("{{css}}", &css)
      .replace("{{body}}", &markdown_html);

    let mut outfile = std::fs::File::create(options.output.join("post.html"))?;
    outfile.write_all(garnished_html.as_bytes())?;
  } else {
    let mut outfile = std::fs::File::create(options.output.join("post.html"))?;
    outfile.write_all(markdown_html.as_bytes())?;
  }

  Ok(CompilationDetails { meta })
}