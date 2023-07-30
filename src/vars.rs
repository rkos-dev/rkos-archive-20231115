use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "rbuilder")]
#[command(author = "xyyy <xyyy1420@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about="Mange rkos package and build rkos",long_about=None)]
pub struct Cli {
    #[arg(long)]
    node: String,
    #[arg(long)]
    graph: String,
}
