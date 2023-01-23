use clap::{arg, Command};

fn main() {}
    let cli_flags = cli().get_matches;
    match matches.subcommand() {
        Some(("exchanges", sub_matches)) => {
        println!(
            "exchanges"
        );
    }
}

fn cli_flags() {
    Command::new("exchanges")
        .about("The exchanges to bootstrap this alg trader with")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
}
