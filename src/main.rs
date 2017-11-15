extern crate bbb_core;
extern crate clap;
extern crate nom;
extern crate rustyline;

mod interpreter;
mod writer;

use std::str::FromStr;

use clap::{App, Arg, SubCommand};
use interpreter::interpret;
use writer::write;

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
                  .required(true)
                  .takes_value(true)
            ).arg(Arg::with_name("duration")
                  .short("d")
                  .long("duration")
                  .value_name("DURATION")
                  .help("Duration of the audio to be generated")
                  .required(true)
                  .validator(is_float)
                  .takes_value(true)
            ).arg(Arg::with_name("expression")
                  .value_name("EXPRESSION")
                  .help("Bytebeat expression from which audio will be generated")
                  .required(true)
                  .takes_value(true)
            )
        )
        .get_matches();

    if args.is_present("repl") {
        interpret().unwrap();
    } else {
        args.subcommand_matches("write").and_then(|write_args| {
            let file = write_args.value_of("output_file").unwrap();
            let expr = write_args.value_of("expression").unwrap();
            let dur = write_args.value_of("duration")
                .and_then(|s| f32::from_str(s).ok())
                .unwrap();

            write(file, dur, expr).ok()
        }).unwrap();
    }
}

fn is_float(v: String) -> Result<(), String> {
    f32::from_str(&*v).map(|_| ()).map_err(|_| "Not a number!".to_owned())
}
