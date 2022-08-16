use clap::{Args, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
enum IndexType {
    Timestamp,
    Sequential,
}

impl From<IndexType> for crate::adr::IndexType {
    fn from(item: IndexType) -> crate::adr::IndexType {
        match item {
            IndexType::Timestamp => crate::adr::IndexType::Timestamp,
            IndexType::Sequential => crate::adr::IndexType::Sequential,
        }
    }
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[clap(help = "The directory where the adrs will live")]
    dir: std::path::PathBuf,
    #[clap(
        short,
        long,
        value_parser,
        help = "The name that will be given to the adr directory"
    )]
    name: String,
    #[clap(short, long, value_enum, default_value = "timestamp")]
    index_type: IndexType,
}

impl InitArgs {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let h = crate::adr::handler::init::Handler{};
        h.handle(&self.dir, self.name, self.index_type.into())
    }
}
