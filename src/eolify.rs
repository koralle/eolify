use std::fs;
use std::io::Write;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use crate::command::{Cli, NewLine};

pub fn eolify(args: &Cli) -> anyhow::Result<()> {
    let root_path = &args.path;

    match walk(&args.path, args.hidden, &args.ignore, args.write) {
        Ok(entries) => {
            for entry in entries.into_iter() {
                let contents = fs::read_to_string(entry.path())?;
                let replaced = replace(&contents, &args.newline)?;

                if args.write {
                    let mut file = fs::File::create(entry.path())?;
                    file.write_all(replaced.as_bytes())?;
                } else {
                    println!("{}", replaced);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn walk(
    path: &PathBuf,
    hidden: bool,
    ignore: &Vec<PathBuf>,
    write: bool,
) -> anyhow::Result<Vec<DirEntry>> {
    Ok(WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect())
}

fn replace(contents: &str, eol: &NewLine) -> anyhow::Result<String> {
    if eol == &NewLine::LF {
        Ok(contents.replace("\r\n", "\n"))
    } else {
        todo!("Not implemented")
    }
}

#[cfg(test)]
mod tests {}
