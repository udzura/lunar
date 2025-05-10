# Lunar: Lua is Not A Ruby!

**Lunar** is a compiler intended to allow Lua scripts to be executed in Ruby.

It converts Lua scripts into [mruby](https://mruby.org/) code and executes them on the mruby VM.

## Install

Install the `lunar-lang` crate published on crates.io. _Please note that the crate named `lunar` is unrelated._

```console
$ cargo install lunar-lang
```

## Usage

```console
$ lunar -h
Lunar: A Lua-to-mruby compiler

Usage: lunar [COMMAND]

Commands:
  compile  Compile a Lua source file to an mruby binary
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### How to Compile Lua Script

```console
$ bat example/hello.lua
bat examples/hello.lua
───────┬──────────────────────────────────────────────────────
       │ File: examples/hello.lua
───────┼──────────────────────────────────────────────────────
   1   │ for i = 1, 5 do
   2   │     print "hello, world\n"
   3   │ end
───────┴──────────────────────────────────────────────────────

$ lunar compile example/hello.lua
$ mruby example/hello.mrb
hello, world
hello, world
hello, world
hello, world
hello, world
```

## Important Notes

Only very basic Lua features are supported. There is no guarantee that all Lua syntax and features will be supported in the future.

For now, it's just a toy program :)

## License

Please see [LICENSE](https://www.google.com/search?q=./LICENSE).