use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    period: u8,
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    connect: Option<String>,
}

fn main() {
    // Get commandline parameters
    let opt = Opt::from_args();
    println!("{:?}", opt);

    //
}
