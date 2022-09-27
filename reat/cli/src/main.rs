use clap::Command;

fn main() {
    let cli = Command::new("reat").arg_required_else_help(true);

    let _matches = cli.get_matches();

    // match matches.subcommand()
}
