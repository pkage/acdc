# ⚡️ acdc

A [reverse-polish](https://xkcd.com/645) desk calculator, in the vein of
[`dc`](https://linux.die.net/man/1/dc). In fact, I wrote it because I use `dc`
quite a bit and wanted to see what it would look like with a slightly nicer UI
and some more features. Definitely not complete, but usable.

## installation

```sh
$ git clone git@github.com:pkage/acdc && cd acdc
$ cargo build --release
$ cp target/release/acdc ~/.local/bin
```

*or elsewhere on your `$PATH`.*

## usage

To enter the prompt:

```sh
$ acdc
alternating current desk calculator v0.1.0
operating in floating point mode, with emacs keybinds.
>>
```

Available operations:

code   | effect
---    | ---
`l`    | print stack length
`f`    | print stack
`p`    | peek at top of stack
`d`    | duplicate top of stack
`P`    | pop top of stack
`c`    | drop stack
`+`    | add the top two elements, push onto stack
`-`    | subtract the top two elements, push onto stack
`*`    | multiply the top two elements, push onto stack
`/`    | divide the top two elements, push onto stack
`^`    | raise the second element to the power of the first, push onto stack
`root` | take the root of the second element (base of the first), push onto stack
`log`  | take the log of the second element (base of the first), push onto stack
`PI`   | (constant) pi
`TAU`  | (constant) tau
`E`    | (constant) E

Other commands:

command | effect
---     | ---
`help`  | show the help
`?`     | show the help
`q`     | quit
`quit`  | quit

Useful keybinds:

*for a full list, [see here](https://github.com/kkawakam/rustyline#emacs-mode-default-mode)*

keybind  | effect
---      | ---
`ctrl-l` | clear the screen
`ctrl-a` | beginning of line
`ctrl-c` | quit immediately
`ctrl-d` | quit immediately
