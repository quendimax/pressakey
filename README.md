# pressakey

[![Crates.io Version](https://img.shields.io/crates/v/pressakey)](https://crates.io/crates/pressakey)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/pressakey)](https://crates.io/crates/pressakey)
[![License](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

## Why this app exists?

There is no a crossplatform **Press any key** tool that would be possible to use
with a shell you prefer (at least I couldn't find that kind of tool). Now in
different shells different tools/commands are used, e.g. `read` in bash and
fish, `pause` in cmd.exe, etc. But even `read` in different shells has different
behaviour of its parameters.

So, I wrote this very primitive app that behaves in the same way on all
available platforms.

## How to use?

Just run the app in your terminal, and it will print the message `Press any
key...`. After you press any key (except for modifier keys), the app will return
control to your shell.

- `-p, --prompt` option allows you to change the message.
- `-e, --expect` specifies the symbols typing that the app return controll to
  the shell. Note that the symbols depend on you language layout; thay are not
  key or scan codes.
- `-s, --silent` allow to suppress the prompt message entirely.
