use clap::Args;

#[derive(Debug, Args)]
pub struct LintArgs {
    path: std::path::PathBuf,
    #[clap(short, long, help = "Recursively search and fmt markdown")]
    recursive: bool,
}

impl LintArgs {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
