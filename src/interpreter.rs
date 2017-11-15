use std::str;
use std::result::Result;
use std::sync::{Arc, Mutex};

use bbb_core::expr::Expr;
use bbb_core::parser::parse;
use bbb_core::player::Player;
use bbb_core::wav::Recorder;
use bbb_core::signal::ExprSignal;

use nom::*;
use rustyline::*;

type CommandResult = Result<Success, String>;

enum Success {
    Write(String),
    Play,
    Stop,
    Quit,
}

enum Command {
    Stop,
    Quit,
    Play { duration: Option<f32>, expr: Expr },
    Write {
        file_name: String,
        duration: f32,
        expr: Expr,
    },
}

struct Environment {
    player: Player,
}

impl Environment {
    fn new() -> Result<Self, String> {
        Player::new(8_000f64, 256)
            .map(|player| Environment { player: player })
            .map_err(|e| format!("{}", e))
    }

    fn play(&mut self, expr: Expr) -> Result<(), String> {
        self.player.play(
            Arc::new(Mutex::new(ExprSignal::from(expr))),
        )
    }

    fn write(&self, file_name: String, duration: f32, expr: Expr) -> Result<String, String> {
        Recorder::new(8_000)
            .record(&*file_name, duration, &mut ExprSignal::from(expr))
            .and(Ok(file_name))
    }
}

fn perform(cmd: Command, env: &mut Environment) -> CommandResult {
    match cmd {
        Stop => {
            env.player.stop().and(Ok(Success::Stop)).map_err(
                |e| format!("{}", e),
            )
        }
        Quit => {
            env.player.stop().and(Ok(Success::Quit)).or(
                Ok(Success::Quit),
            )
        }
        Play { duration, expr } => env.play(expr).and(Ok(Success::Play)),
        Write {
            file_name,
            duration,
            expr,
        } => env.write(file_name, duration, expr).map(Success::Write),
    }
}

pub fn interpret() -> Result<(), String> {
    let mut repl = Editor::<()>::new();
    Environment::new().map(|mut env| loop {
        let input = repl.readline(" ∿ ").map_err(|_| {
            "Could not read command, try again".to_owned()
        });

        match input {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                } else {
                    let cmd = command(line.as_bytes()).to_result().map_err(|_| {
                        format!("\"{}\" is not a valid command", line)
                    });

                    let result = cmd.and_then(|command| perform(command, &mut env));

                    match result {
                        Ok(Success::Quit) => break,
                        Ok(Success::Write(file_name)) => println!("file saved to: {}", file_name),
                        Err(e) => println!("{}", e),
                        _ => continue,
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    })
}

/* ------------- PARSING */
use self::Command::*;

named!(file_name<String>, map_res!(
    tuple!(
        many1!(alphanumeric),
        tag!(".wav")
    ),
    |(name, extension): (Vec<&[u8]>, &[u8])| {
        name.iter().map(|&bs| str::from_utf8(bs)).fold(Ok(String::new()), |name, part| {
            name.and_then(|s1| part.map(|s2| s1+ s2))
        }).and_then(|n| {
            str::from_utf8(extension).map(|e| format!("{}{}", n, e))
        })
    }
));

named!(command<Command>, ws!(alt!(stop | quit | play | write)));
named!(stop<Command>, value!(Stop, tag!("stop")));
named!(quit<Command>, value!(Quit, tag!("quit")));

named!(play<Command>, do_parse!(play_command >> cmd: play_args >> (cmd)));
named!(play_command<&[u8]>, recognize!(tag!("play")));
named!(play_args<Command>, map!(
    pair!(opt!(float), ws!(parse_expr)),
    |(dur, exp)| Play { duration: dur, expr: exp }
));

named!(write<Command>, do_parse!(write_command >> cmd: write_args >> (cmd)));
named!(write_command<&[u8]>, recognize!(tag!("write")));
named!(write_args<Command>, map!(
    tuple!(ws!(file_name), ws!(float), ws!(parse_expr)),
    |(fname, dur, expr)| Write { file_name: fname, duration: dur, expr: expr }
));

named!(parse_expr<Expr>, map_res!(
    call!(rest),
    |byte| str::from_utf8(byte).map_err(|e| format!("{}", e)).and_then(parse)
));
