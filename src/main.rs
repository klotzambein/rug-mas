use std::{error::Error, path::PathBuf};

use clap::{AppSettings, Clap};
use config::Config;
// use plotters::{
//     prelude::{BitMapBackend, IntoDrawingArea},
//     style::WHITE,
// };
use report::Reporter;
use simulation::Simulation;
use toml::to_string_pretty;

pub mod agent;
pub mod config;
pub mod market;
pub mod report;
pub mod simulation;

/// Application to investigate market behavior in gossiping agents.
#[derive(Clap)]
#[clap(version = "0.1", author = "Robin Kock <contact@robin-kock.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Run(RunCommand),
    WriteConfig(WriteConfigCommand),
}

/// Run a simulation.
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct RunCommand {
    /// Path to configuration file. Default configuration can be written to a
    /// file using 'rug-mas write-config ./default.toml'
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// The length of each simulation run in steps.
    #[clap(short = 'n', long, default_value = "10000")]
    run_length: usize,

    /// The length of each simulation run in steps.
    #[clap(short, long, default_value = "1")]
    repetitions: u32,
}

/// Export the default configuration.
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct WriteConfigCommand {
    /// Path to new configuration file.
    config: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opts::parse();

    match args.subcmd {
        SubCommand::Run(rc) => run_simulation(rc),
        SubCommand::WriteConfig(wc) => write_config(wc),
    }
}

fn write_config(cmd: WriteConfigCommand) -> Result<(), Box<dyn Error>> {
    let config = to_string_pretty(&Config::default())?;
    std::fs::write(cmd.config, &config)?;
    Ok(())
}

fn run_simulation(cmd: RunCommand) -> Result<(), Box<dyn Error>> {
    let config = cmd
        .config
        .as_ref()
        .map(Config::load)
        .unwrap_or_else(|| Ok(Config::default()))?;

    for _run_index in 0..cmd.repetitions {
        let mut reporter = Reporter::new();
        let mut sim = Simulation::new(&config);
        for step in 0..cmd.run_length {
            reporter.set_step(step);
            sim.step(step, &mut reporter);
        }
        // let drawing_area = BitMapBackend::new("plot.png", (1024, 512)).into_drawing_area();
        // drawing_area.fill(&WHITE).expect("Can't fill bitmap");
        // reporter.render_chart(drawing_area);
        let mut name = cmd
            .config
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .and_then(|n| n.split('.').next())
            .unwrap_or("result")
            .to_owned();
        name += ".csv";
        reporter.write_csv(name)
    }

    Ok(())
}
