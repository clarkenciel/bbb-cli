use std::str;

use bbb_core::expr::Expr;
use bbb_core::parser::parse;
use bbb_core::player::Player;
use bbb_core::wav::Recorder;
use bbb_core::signal::ExprSignal;

use nom::*;
use rustyline::*;

pub fn interpret() {
    let mut repl = Editor::<()>::new();
    let mut env = Environment::new();
    loop {
        let result = repl.readline(" ∿ ").and_then(|line| command(line).to_result());
        match result {
            Ok(Quit) => break,
            Ok(cmd) => perform_command(&env, cmd),
            Err(e) => println!("Sorry, \"{}\" is not a valid command", e),
        }
    }
}

fn perform_command<'a>(env: &Environment, cmd: Command<'a>) {
    match cmd {
        Stop => env.stop(),
        Quit => return,
        Write { file_name, duration, expr } => write_file(file_name, duration, expr),
        Play { duration, expr } => play_audio(&mut env.player, duration, expr),
    }
}

fn write_file<'a>(file_name: &'a str, duration: f32, expr: Expr) {
}

fn play_audio(player: &mut Player, duration: Option<f32>, expr: Expr) {
}

enum Command<'a> {
    Stop,
    Quit,
    Play { duration: Option<f32>, expr: Expr },
    Write {
        file_name: &'a str,
        duration: f32,
        expr: Expr,
    },
}

struct Environment {
    player: Player,
}

impl Environment {
    fn new() -> Self {
    }
}

/* ------------- PARSING */
use self::Command::*;

named!(file_name<&str>, map_res!(take_until_s!(" "), str::from_utf8));
named!(command<Command>, ws!(alt!(stop | quit | play | write)));
named!(stop<Command>, value!(Stop, tag!("stop")));
named!(quit<Command>, value!(Quit, tag!("quit")));
named!(play<Command>, map!(
    pair!(opt!(float), ws!(parse_expr)),
    |(dur, exp)| Play { duration: dur, expr: exp }
));

named!(write<Command>, map!(
    tuple!(file_name, ws!(float), ws!(parse_expr)),
    |(fname, dur, expr)| Write { file_name: fname, duration: dur, expr: expr }
));

named!(parse_expr<Expr>, map_res!(
    call!(rest),
    |byte| str::from_utf8(byte).map_err(|e| format!("{}", e)).and_then(parse)
));
