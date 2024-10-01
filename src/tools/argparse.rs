use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    #[arg(long, default_value_t=80, value_name = "HTTP PORT")]
    pub http: u16,

    #[arg(long, default_value_t=443, value_name = "HTTPS PORT")]
    pub https: u16,
}
