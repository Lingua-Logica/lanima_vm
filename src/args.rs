use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "LanimaVm",
    version = "0.0.1",
    about = "Building a Rust-style variant of LMVM",
    long_about = None
)]
pub struct Args {
    /// 输入字节码文件路径（可选）
    #[arg(short, long)]
    pub(crate) bytecode: Option<String>,
}
