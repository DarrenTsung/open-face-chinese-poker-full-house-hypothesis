use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "full-house-hypothesis-tester",
    about = "https://github.com/DarrenTsung/open-face-chinese-poker-full-house-hypothesis"
)]
pub struct Opt {
    /// The number of runs used to calculate probability. Defaults to 1,000.
    #[structopt(long = "runs", default_value = "1000")]
    runs: usize,
}
