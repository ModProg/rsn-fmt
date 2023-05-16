use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use clap::Parser;
use dirs_next::config_dir;
use figment::providers::{Env, Format};
use figment::Figment;
use rsn_fmt::Config;

type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
struct Opts {
    files: Vec<PathBuf>,
    #[arg(long)]
    config: Option<PathBuf>,
}

fn config_from_dir(path: impl AsRef<Path>) -> Figment {
    let path = path.as_ref();
    Figment::new()
        .merge(Rsn::file(path.join(".rsnfmt.rsn")))
        .merge(Rsn::file(path.join("rsnfmt.rsn")))
}

fn load_config(path: Option<&Path>) -> Result<Config, figment::Error> {
    let mut config = Figment::new().merge(Env::prefixed("RSNFMT_"));
    if let Some(path) = path {
        config = config.merge(Rsn::file(path));
    }
    {
        let config: Config = config.extract()?;
        if !config.inherit {
            return Ok(config);
        }
    }
    for dir in current_dir()
        .as_deref()
        .map(Path::ancestors)
        .into_iter()
        .flatten()
    {
        config = config.join(config_from_dir(dir));
        let config: Config = config.extract()?;
        if !config.inherit {
            return Ok(config);
        }
    }
    let config_dir = config_dir().expect("Supported OS should yield config dir");
    Figment::new()
        .join(Rsn::file(config_dir.join(".rsnfmt.rsn")))
        .join(Rsn::file(config_dir.join("rsnfmt.rsn")))
        .extract()
}

struct Rsn;

impl Format for Rsn {
    type Error = rsn::de::Error;

    const NAME: &'static str = "RSN";

    fn from_str<T: serde::de::DeserializeOwned>(string: &str) -> Result<T, Self::Error> {
        rsn::from_str(string)
    }
}

fn main() -> Result {
    let opts = Opts::parse();
    let config = load_config(opts.config.as_deref())?;
    for file in &opts.files {
        fs::write(
            file,
            rsn_fmt::format_str(
                &fs::read_to_string(file)
                    .with_context(|| format!("reading `{}`", file.display()))?,
                &config,
            )
            .with_context(|| format!("formatting `{}`", file.display()))?,
        )
        .with_context(|| format!("writing `{}`", file.display()))?;
    }
    Ok(())
}
