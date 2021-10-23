# bc

I liked using [`bc`](https://en.wikipedia.org/wiki/Bc_(programming_language)) when I was on Ubuntu. I'm cloning it in Rust so that I'll be able to use it in every OS I use. 

Also, learning interpreters / compilers.

# TODO
- [x] Tokenizer
- [x] AST generation
  - [x] Parser / Compiler rewrite
- [x] Actual interpretation (stack machine)
- [x] Global variables
- [ ] Proper error handling.
- [ ] Big-Number Arithmetics (`scale`)
- [ ] Functions (`define`)
  - [ ] Local environment for local variables?
  - [ ] BUG: Check the arity of the called function. For example `sqrt(5, 4)` works and generates a `PushConstant(4)` instruction!
- [ ] Standard Library
- [ ] UTF-8 support
- [ ] Support for econometry stuff?
  - [ ] Arrays
  - [ ] Statistical Functions
  - [ ] What else?
