# `rlox`

## Overview

`rlox` is an implementation in Rust of the `jlox` tree-walker interpreter from [_Crafting Interpreters_](https://craftinginterpreters.com/) by [Robert Nystrom](https://twitter.com/intent/user?screen_name=munificentbob).

My motivation for choosing Rust is twofold:

- I'm currently trying to learn Rust in my free time and I thought using it to write my own `jlox` would force me to work through real examples of concepts that are usually always explained with contrived examples. Hopefully, this leads to a deeper understanding of Rust concepts sooner.
- Using a language other than Java forces me to stop and think about the concept in a way that I might not otherwise if I were merely transcribing the original source code from the book. Hopefully, this means the concepts stick with me.

## Goals

- Obviously, arrive at a complete implementation of `jlox` in Rust
- Take note of blockers in Rust and explain
- Write tests
- Add documentation
