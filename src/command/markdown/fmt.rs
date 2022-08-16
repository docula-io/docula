use clap::Args;

#[derive(Debug, Args)]
pub struct FmtArgs {
    path: std::path::PathBuf,
    #[clap(short, long, help = "Recursively search and fmt markdown")]
    recursive: bool,
    #[clap(long, help = "Print the output without making it")]
    dry_run: bool,
}

impl FmtArgs {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let h = crate::markdown::handler::fmt::Handler{};
        h.handle(&self.path)
    }
}
