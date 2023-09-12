use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    time::Instant,
};

use anyhow::Result;
use clap::Parser;
use compact_str::CompactString;
use crossbeam_channel::unbounded;
use exchange::{
    engine::Engine,
    order::{util::DEFAULT_PAIR, OrderRequest},
};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Layer, Registry};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value = DEFAULT_PAIR, help = "Pair")]
    pair: CompactString,
    #[clap(short, long, value_parser = clap::value_parser!(Input), help = "Source of Order requests")]
    input: Option<Input>,
    #[clap(short, long, value_parser = clap::value_parser!(Output), help = "Target of Order Book events")]
    output: Option<Output>,
}

#[derive(Debug, Default, Clone)]
enum Input {
    #[default]
    Stdin,
    File(PathBuf),
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::File(s.to_owned().into()))
    }
}

#[derive(Debug, Default, Clone)]
enum Output {
    #[default]
    Stdout,
    File(PathBuf),
}

impl FromStr for Output {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::File(s.to_owned().into()))
    }
}

fn main() -> Result<()> {
    // Initialize logging
    let _guard = init_logs();

    info!("Matching engine started!");

    // Parse command line arguments
    let args = Args::parse();

    let (tx, rx) = unbounded();

    // Start reading orders in a separate thread
    let reader = read(args.input.unwrap_or_default(), tx);
    reader.join().expect("order reader thread panicked")?;

    // Create the matching engine
    let mut engine = Engine::new(&args.pair);

    // Process all the order requests
    process(&mut engine, rx)?;

    // Report results
    info!("Matching engine finished!");
    report(args.output.unwrap_or_default())?;

    Ok(())
}

fn init_logs() -> WorkerGuard {
    LogTracer::init().expect("Unable to set up log tracer");

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_layer = fmt::layer()
        .json()
        .with_thread_names(true)
        .with_writer(non_blocking_writer)
        .with_filter(EnvFilter::from_default_env());

    let subscriber = Registry::default().with(stdout_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    guard
}

fn read(input_source: Input, tx: crossbeam_channel::Sender<OrderRequest>) -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(move || -> Result<()> {
        let mut buf_read: Box<dyn BufRead> = match &input_source {
            Input::File(path) => {
                let file = std::fs::File::open(path)?;
                Box::new(BufReader::new(file))
            }
            Input::Stdin => {
                let stdin = std::io::stdin();
                Box::new(BufReader::new(stdin))
            }
        };

        let mut buf = String::with_capacity(4096);
        while buf_read.read_line(&mut buf).is_ok() {
            let order = serde_json::from_str(&buf);
            buf.clear();
            match order {
                Err(error) => {
                    if error.is_eof() {
                        break;
                    }
                    error!("Error processing source of orders: {}", error);
                }
                Ok(order) => tx.send(order)?,
            }
        }

        Ok(())
    })
}

fn process(engine: &mut Engine, rx: crossbeam_channel::Receiver<OrderRequest>) -> Result<()> {
    let mut i = 0.0;
    let start = Instant::now();

    while let Ok(order_request) = rx.recv() {
        if let Err(error) = engine.process(order_request) {
            error!("Error processing order request: {}", error);
        }
        i += 1.0;
    }

    let end = Instant::now();
    let elapsed = end - start;

    info!("Matching engine finished!");
    info!(
        "{:.0} order requests processed in {} milliseconds",
        i,
        elapsed.as_millis()
    );

    Ok(())
}

fn report(output_destination: Output) -> Result<()> {
    match output_destination {
        Output::Stdout => (),
        Output::File(..) => unimplemented!(),
    }

    Ok(())
}