### Rustypop

This is a not-yet-functional port of lalr-parser.y, from the Rust
repository, into LALRPOP. As of the moment, I am still in the process
of working my way through shift-reduce failures.

Eventually, I am aiming for the following conventions:

1. Terminal names will use `""` if they are keywords or other
   literals, otherwise the terminal will be named with camel-case like
   `Ident`.
2. For multi-character symbols that can also be interpreted as single characters,
   like `&&`, `<<`, or `||`, we will introduce two terminals:
   
   - One for the character when following by another of the same character, written with
     a `+`, e.g. `<+`.
   - One for the character alone, e.g., `<`.
   
   Therefore, to match `<<`, you would match `"<+" "<"`. In cases,
   like types, where `<<` is to be considered as two distinct `<`
   tokens, we would match either `"<+"` OR `"<"` equivalently.
3. Nonterminal names will be camel-case like `Expr`.
4. Variants on expressions and the like will be implemented with
   LALRPOP macros, as will comma-separated lists (`Comma<V>`) and so
   forth.  We can also employ `Foo?` instead of conventions like `maybe_foo`.
   
The easiest way to experiment right now is to build `lalrpop-exe` from
the LALRPOP project, and then to run `lalrpop-exe -f
parser-lalr.lalrpop`.

### Licensing

Licensed under the same terms as Rust itself: dual MIT and Apache2.
