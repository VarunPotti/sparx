use config::Config;
use log::{debug, info};

use crate::{cli::build_cli, commands::init::init};

mod cli;
mod commands;
mod config;
mod utils;

async fn try_main() -> anyhow::Result<()> {
    let config = Config::new()?;

    let matches = build_cli()?;

    match matches.subcommand() {
        Some(("init", matches)) => {
            debug!("init command");

            let template = matches.get_one::<String>("template-name").unwrap();
            let name = matches.get_one::<String>("name").unwrap();
            let directory = matches.get_one::<String>("directory").unwrap();
            let variables = matches.get_raw("flags").unwrap_or_default();

            // convert all variables to a hashmap
            let mut variables_map = std::collections::HashMap::new();
            for variable in variables {
                let mut split = variable.to_str().unwrap().split('=');
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                variables_map.insert(key.to_string(), value.to_string());
            }

            init(
                template.to_string(),
                name.to_string(),
                directory.to_string(),
                variables_map,
                config.clone(),
            )
            .await?;
        }
        _ => {
            debug!("No command specified");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "debug");

        env_logger::init();
    }

    if let Err(e) = try_main().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
