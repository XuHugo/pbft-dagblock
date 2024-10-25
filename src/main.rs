use blockdag::test_pfbf_dag;
use clap::{Arg, Command};
use pbft_blockdag::network::launcher::{self, test_double_pbft};
fn main() {
    let matches = Command::new("xxxNetwork")
        .version("1.0.0")
        .author("xxxxx")
        .about("A implementation of PBFT+DPOS consensus. 🦀")
        .arg(
            Arg::new("f")
                .short('f')
                .long("number of normal nodes")
                .value_parser(clap::value_parser!(u32))
                .help("Sets the number of normal nodes"),
        )
        .arg(
            Arg::new("n")
                .short('n')
                .long("number of consensus nodes")
                .value_parser(clap::value_parser!(u32))
                .help("Sets the number of consensus nodes"),
        )
        .get_matches();

    let f = *matches.get_one::<u32>("f").unwrap_or(&1);
    let n = *matches.get_one::<u32>("n").unwrap_or(&4);

    if n < f {
        panic!("The number of nodes must be greater than the number of faulty nodes.");
    }

    println!("f: {}", f);
    println!("n: {}", n);

    //pbft nodes network
    launcher::start_nodes_pbft(n).unwrap();

    //dag nodes network
    blockdag::start_nodes_dag(f as i32);

    //test times
    //test_double_pbft();
    //launcher::start_nodes_pbft(4).unwrap();
    //test_pfbf_dag();
}
