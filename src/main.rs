use std::{error::Error, path::PathBuf, thread};

use clap::{AppSettings, Clap};
use config::Config;
use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::WHITE,
};
// use plotters::{
//     prelude::{BitMapBackend, IntoDrawingArea},
//     style::WHITE,
// };
use report::Reporter;
use simulation::Simulation;
use speedy2d::{Window, window::{UserEventSender, WindowCreationOptions, WindowSize}};
use toml::to_string_pretty;
use window::Data;

pub mod agent;
pub mod config;
pub mod market;
pub mod report;
pub mod simulation;
pub mod window;

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

    /// Create a plot of the reported values.
    #[clap(short, long)]
    plot: bool,

    /// Write a csv with the reported values.
    #[clap(long)]
    csv_write: bool,

    /// Open a window and show a visualization while the simulation is running.
    #[clap(short, long)]
    window: bool,
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

    if cmd.window {
        let size = WindowSize::MarginPhysicalPixels(100);
        let opts = WindowCreationOptions::new_windowed(size, None);
        let window = Window::<Data>::new_with_user_events("Title", opts).unwrap();
        let event_sender = window.create_user_event_sender();

        thread::spawn(move || sim_loop(cmd, config, Some((1, event_sender))));

        window.run_loop(window::MyWindowHandler::default());
    } else {
        sim_loop(cmd, config, None);
    };

    Ok(())
}

fn sim_loop(cmd: RunCommand, config: Config, event_sender: Option<(usize, UserEventSender<Data>)>) {
    for _run_index in 0..cmd.repetitions {
        let mut reporter = Reporter::new();
        let mut sim = Simulation::new(&config);
        for step in 0..cmd.run_length {
            reporter.set_step(step);
            sim.step(step, &mut reporter);
            
            if let Some((update_rate, es)) = &event_sender {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                if step % update_rate == 0 {
                    let data = Data {
                        sim: sim.clone(),
                        report: reporter.clone(),
                    };
                    es.send_event(data).unwrap();
                }
            }

        }
        if cmd.plot {
            let drawing_area = BitMapBackend::new("plot.png", (10240, 5120)).into_drawing_area();
            drawing_area.fill(&WHITE).expect("Can't fill bitmap");
            reporter.render_chart(drawing_area);
        }
        if cmd.csv_write {
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
    }
}
