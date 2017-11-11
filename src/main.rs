extern crate bbb_core;
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let args = App::new("bbb-cli")
        .version("1.0.0")
        .about(
            "Barely featured bytebeat equation interpreter and wave file generator.",
        )
        .author("Danny Clarke")
        .subcommand(SubCommand::with_name("repl").about(
            "Open a simple bytebeat interpreter.",
        ))
        .subcommand(
            SubCommand::with_name("write").about(
                "Generate a wave file using audio generated from a given bytebeat equation."
            ).arg(Arg::with_name("output_file")
                  .short("o")
                  .long("output-file")
                  .value_name("OUTPUT FILE")
                  .help("File to which audio will be written")
                  .takes_value(true)
            ).arg(Arg::with_name("duration")
                  .short("d")
                  .long("duration")
                  .value_name("DURATION")
                  .help("Duration of the audio to be generated")
                  .takes_value(true)
            ).arg(Arg::with_name("expression")
                  .value_name("EXPRESSION")
                  .help("Bytebeat expression from which audio will be generated")
                  .takes_value(true)
            )
        )
        .get_matches();

    println!("{:?}", args);
}
