# 💀 RUST REBEL

Rust Rebel is the soft layer over Rust’s hard edges, translating the language’s often terse, academic naming conventions into accessible, intuitive terms.

Rust’s core vocabulary is powerful but unwelcoming. Concepts that have obvious idioms in other languages are wrapped in unnecessarily esoteric names.

Rebel exists to normalize those terms, patterns, and mental anchors so you don’t need a PhD or a fetish for category theory to feel at home.
Many concepts in Rust ship without strong domain metaphors; Rebel fills that gap by naming things from first principles, and throws Rustiness out the window when it gets in the way of mental modeling.

Rebel is the anomaly in Rust’s perfect ecosystem — the stuff no one tells you.

We have absolutely **nothing** to do with the Rust Foundation, besides bending Rust to our will.


## Un-Named Module Patterns

Rust doesnt provide a friendly mental anchor to hang your hat on here; so Rebel takes the liberty of naming them on behalf of everyone:

> Naive Mod Pattern (as in I have no idea how to design a modular system)

Using `mod.rs` inside of a directory to  **hoist** faces, types and functions into the parent scope. This pattern is older but still supported.

```bash
mod_name/
 ├─ mod.rs
 ├─ foo.rs
 └─ bar.rs
```
Inside `mod.rs`

```rust
// mod_name/mod.rs
pub mod foo;
pub mod bar;
```

> Nice Neighbor Pattern 

Using a sibling file `name.rs` that matches a directory `name`, the name.rs file acts as mod.rs as does in the older pattern. But unlike mod.rs it is outside of the folder lol. They did this so you dont have to shuffle through a dozen mod files. An improvement, but also not ideal.

```bash
mod_name.rs
mod_name/
 ├─ foo.rs
 └─ bar.rs
```
Inside `name.rs`

```rust
// mod_name.rs
// an improvement but still suffers from neighbors telling you how to decorate your house
pub mod foo;
pub mod bar;
```

> 💀 Rebel Smart Pattern (A Proposal)

In an ideal world you wouldn't need a file to prime the hoisting mechanism; the compiler should be smarter about what needs to be exported.


```bash
mod_name/
 ├─ foo.rs
 └─ bar.rs
```
Inside `foo.rs`, it would be hot if files could just define their own exports. They have the power of the file boundary, and within that boundary it would make a hella of a lot more sense if files  decide what to shape and reveal rather than having some other external file do it for them. Be gone, you have no power here external file!


```rust
// mod_name/foo.rs
pub export Foo, foo_stats;
pub super export FooConfig;

```


## Shitty Functions, Diva Compiler

Some times our brains are just way too big for the entire universe to contain them all, not to mention the infinite layers of meta data needed to describe our unfathomable intelligence.

> Function implements Violence

```rust
// This is nothing less than utter violance against human cognition. 
// no seriously. what is this shit?
fn process_data<T, E>(
    input: &mut Vec<Option<Result<T, E>>>,
    processor: impl Fn(&T) -> Result<T, E>,
    filter: Option<&dyn Fn(&T) -> bool>
) -> Result<Vec<T>, ProcessingError<E>>
```

This is arguably one of many things keeping people from learning Rust. It is a dyslexic nightmare. 

Sure, if you have to stare at insanity like this for 16 hours a day you *might* think this was normal, but I'd argue that one of the fundamnetal principles of good software design is **legibility**. 

A professor I deeply respect once said and I quote 

| "The true sign of *intelligence* is doing more with **less**." 

Elegance and simplicity are not conveniences, but rather obvious signs of a well designed system.
On that note, let's turn our attention to the diva herself: the compiler. 

Why is the Rust compiler such a princess that she needs you to show up in full gown, makeup and heels for a casual rendevouz? And then to add insult to injury she has the audacity to tell you how to curtsie correctly if you skip a beat.


```rust

error[E0308]: mismatched types
  --> src/main.rs:42:5
   |
42 |     return arg;
   |            ^^^ expected `Result<Context, ContextError>` but found `Context`
   |
help: try wrapping the expression in `Ok`:
   |
42 |     return Ok(arg);
   |            +++   +

```
"Oh, you didn't wrap it in Ok()? Let me explain proper error handling etiquette to you, peasant."

Some people think the compiler is being "nice", if it were nice it would fix it for me and stop harrassing me. Rust is ridiculous.



> 💀 Rebel Function (A Proposal)

Isn't this divine? Just one blob, thats all I want, do your crazy shit in the function not outside of it. Yes, yes, yes type checking. Thats fine this doesnt eskew type checking, it require a thousand less brain cells firing at the same time to make sense of. 

Makes functional programming (chaining) hella easier for free. Who doesnt love free shit?

```rust

//Context implements the Argument trait and a Result trait lol

fn process_data( arg: Context ... ) -> Context {

  //do stuff to args innards
  return arg;

}

```
Big brains are going to akshually the shit out of this, but I got one thing to say, I am not a PhD, but you can call me Dr Vegajunk.