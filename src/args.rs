use clap:: {
    Parser
};

/// Simple program to generate random passwords
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct MyArgs {
    /// Password lenght
    #[clap(short, long, default_value = "5")]
    pub size: usize,

    /// Numbers
    #[clap(short, long, default_value = "true")]
    pub n: String,

    /// Lower case letters
    #[clap(short, long, default_value = "true")]
    pub lcl: String,

    /// Upper case
    #[clap(short, long, default_value = "true")]
    pub ucl: String,

    /// Symbols
    #[clap(short, long, default_value = "false")]
    pub y: String,

    /// Spaces
    #[clap(short, long, default_value = "false")]
    pub k: String,

    /// Exclude similar characters
    #[clap(short, long, default_value = "true")]
    pub c : String,

    /// Strict
    #[clap(short, long, default_value = "false")]
    pub t : String,
}