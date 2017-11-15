# bbb-cli

Bytebeat interpreter/file writer command line tool using the [bbb-core](https://github.com/clarkenciel/bbb-core) "backend."

## building
*NB* this program depends on Portaudio, you may need to have it installed to build.

1. Install [cargo](http://doc.crates.io/), the rust package manager
2. `git clone https://github.com/clarkenciel/bbb-cli`
3. `cargo install --path .`

## usage
`bbb-cli --help` will display more thorough information, but here's a brief run down.
This tool has to utilities/commands: `repl` and `write`.
`repl` will open a sort of interpreter for bytebeat that allows you to `play` and `write` bytebeat equations in real time. Within the repl you can run `help` to see available commands.
`write` will use a file name, a duration, and an equation to create a wave file with audio generated from the equation.

## equations
Generally bytebeat equations look something like this: `t & 96 << 10 + 1`. That is they have access to basic arithmetic and bit-wise operations as well as a global variable `t`, that is a stand-in for "current time/sample." These equations will be evaulated for each sample during audio generation using the current sample count as `t`.
