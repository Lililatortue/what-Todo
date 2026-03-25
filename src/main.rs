mod action;
mod cli;
pub(crate)mod parser;
mod configuration;
mod navigation;

use cli::Cli;
use clap::Parser;

use configuration::{
    workspace_config::WorkSpaceConfig,
    regex_config::RegexConfig
};
//TODO(REFACTOR_CONFIG) {Make COnfig more ergonomic}
pub struct Config {
    workspace: Result<WorkSpaceConfig, &'static str>,
    regex    : RegexConfig,
}


fn main()->Result<(),String> {
    //todo(REFACTOR_CONFIG) { make config static for code clarity }
    //todo(INTEGRATION_TEST) {make integration test}
    let workspace = WorkSpaceConfig::new();
    let config    = workspace.as_ref()
                        .map(|w| w.config.as_ref())
                        .unwrap_or_else(|e| *e);

    let regex = RegexConfig::new(&config)
        .map_err(|e| e.to_string())?;
    
    let config= Config{workspace, regex};

    cli::run(config ,Cli::parse())?;
    Ok(())
}



