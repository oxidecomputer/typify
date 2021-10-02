use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Command `rustfmt` could not be found
    #[error("rustfmt is not installed")]
    NoRustfmt,
    /// Command `rustfmt` produced an error at runtime.
    #[error("rustfmt runtime error")]
    Rustfmt(String),
    /// Error with file IO
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// Error from reading stdin of rustfmt
    #[error(transparent)]
    Conversion(#[from] std::string::FromUtf8Error),
}

pub fn rustfmt<T: ToString>(input: T) -> Result<String, Error> {
    let input = input.to_string();

    let mut builder = tempfile::Builder::new();
    builder.prefix("rustfmt-wrapper");
    let outdir = builder.tempdir().expect("failed to create tmp file");

    let rustfmt_config_path = outdir.as_ref().join("rustfmt.toml");
    std::fs::write(rustfmt_config_path, "")?;

    let rustfmt = which_rustfmt().ok_or(Error::NoRustfmt)?;

    let mut command = Command::new(&rustfmt)
        .arg("--edition=2018")
        .arg(format!("--config-path={}", outdir.path().to_str().unwrap()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = command.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = command.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(Error::Rustfmt(String::from_utf8(output.stderr)?))
    }
}

fn which_rustfmt() -> Option<PathBuf> {
    match env::var_os("RUSTFMT") {
        Some(which) => {
            if which.is_empty() {
                None
            } else {
                Some(PathBuf::from(which))
            }
        }
        None => toolchain_find::find_installed_component("rustfmt"),
    }
}

#[cfg(test)]
mod tests {
    use crate::rustfmt;
    use newline_converter::dos2unix;
    use quote::quote;

    #[test]
    fn test_basics() {
        assert_eq!(
            dos2unix(
                rustfmt(quote! { struct Foo { bar: String }})
                    .unwrap()
                    .as_str()
            ),
            "struct Foo {\n    bar: String,\n}\n"
        );
    }
}
