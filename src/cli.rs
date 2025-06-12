use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Model {
    /// Kontext Max [$0.08 / Image]
    Kontext,
    /// Kontext Max [$0.08 / Image]
    KontextMax,
    /// Kontext Pro [$0.04 / Image]
    KontextPro,
}

#[derive(Parser, Debug)]
#[command(version, about = "Generate Images with the Flux API", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Prompt to generate
    #[clap(short = 'p', long)]
    pub prompt: Option<String>,

    /// The model to use
    #[clap(short = 'm', long)]
    #[arg(value_enum, default_value_t = Model::Kontext)]
    pub model: Model,

    /// Image to provide the model as a path
    #[clap(short = 'i', long)]
    pub image: Option<String>,

    /// Aspect Ratio between 21:9 and 9:21
    #[clap(short = 'a', long)]
    pub aspect_ratio: Option<String>,

    /// Flag to enhance prompt with LLM
    #[clap(short = 'u', long)]
    pub prompt_upsampling: bool,

    /// Prompt to generate
    #[clap(long)]
    pub seed: Option<i32>,

    /// Don't use credits
    #[clap(long)]
    pub dry_run: bool,

    /// Safety tolerance, 0 -> Most strict, 6 -> Least strict
    #[clap(long)]
    #[arg(default_value_t = 6)]
    pub safety: i32,

    /// Input text
    #[clap(trailing_var_arg = true)]
    text: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Gui,
    // Cli,
}

#[derive(Parser, Debug)]
pub struct CommonArgs {
    #[clap(short = 'm', long)]
    pub model: String,
}
