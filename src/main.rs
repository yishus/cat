use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file {:?}", path))?;
    println!("file content: {}", content);
    Ok(())
}
