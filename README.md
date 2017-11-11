# bbb-cli

Bytebeat interpreter/file writer command line tool using the [bbb-core](https://github.com/clarkenciel/bbb-core) "backend."

## usage
`bbb-cli --help` will display more thorough information, but here's a brief run down.
This tool has to utilities/commands: `repl` and `write`.
`repl` will open a sort of interpreter for bytebeat that allows you to `play` and `record` bytebeat equations in real time.
`write` will use a file name, a duration, and an equation to create a wave file with audio generated from the equation.

## equations
Generally bytebeat equations look something like this: `t & 96 << 10 + 1`. That is they have access to basic arithmetic and bit-wise operations as well as a global variable `t`, that is a stand-in for "current time/sample." These equations will be evaulated for each sample during audio generation using the current sample count as `t`.
