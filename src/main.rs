// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use onefetch::{cli::Cli, cli_utils, error::*, git_utils, info};

use {
    process::{Command, Stdio},
    std::{io, process},
};

mod onefetch;

fn run() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    if !is_git_installed() {
        return Err("git failed to execute!".into());
    }

    let config = Cli::new()?;

    if !git_utils::is_valid_repo(&config.repo_path)? {
        return Err("please run onefetch inside of a non-bare git repository".into());
    }

    if config.print_languages {
        return cli_utils::print_supported_languages();
    }

    if config.print_package_managers {
        return cli_utils::print_supported_package_managers();
    }

    let info = info::Info::new(config)?;

    let mut printer = cli_utils::Printer::new(io::BufWriter::new(io::stdout()), info);

    printer.print()?;

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            let stderr = io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
    }
}

fn is_git_installed() -> bool {
    Command::new("git").arg("--version").stdout(Stdio::null()).status().is_ok()
}
