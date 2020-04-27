use structopt::StructOpt;

/// Save web pages to peruse later.
#[derive(StructOpt)]
struct Cli {
    /// The URL to save.
    url: String,
    /// The path on filesystem.
    path: String,
}

fn main() {
    let args = Cli::from_args();
    println!("{} {}", args.url, args.path);
}
