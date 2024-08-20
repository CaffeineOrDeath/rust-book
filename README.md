# Beefed Up Rust Book

What do I mean by beefed up? Take the guessing game and add difficulty levels.
Add difficulty levels using enums. Pull all of the code out into their own
functions. Usage for the first time player, game loop, etc. Yeah, not just your
basic rust book.

## Why make it harder?

Well, how else do you learn? In this regard, breaking the basics apart allows
you start seeing what makes up the core of an application regardless of the
langauge used. It allows you to see where and when an application has an issue
even with some of the crappy unwrap() logs.

Rust can be a fun langauge once you start to a get a feel for its nuances.

## Wait, did you say Rust is fun?

Yes, yes I did. It is only as complex and difficult as you make it. For example,
we can write Hello, World in more than just println! We can use arrays, tuples,
lists, loops, maps, ptrs, bit banging, you name it, you can write hello world
using it, even if it does become an unwieldy mess of speghetti code.

You can use that idea to explore Rust further and see what other ways you can
use it. So far, it's been fun, if a little weird at times.

## So, I can build anything with Rust?

Seems so to me. I'd compare it as TS to JS (Rust to C/C++/C#).

## C, C++, AND C#?

The amount of features that Rust contains makes it comprable to C++ however,
unlike C++, Rust also has a built in package manager, Cargo, that ships packages
called Crates. No need for vcpkg, cmake, make hacks, or any build system. That's
included, too.

I'd say it's comprable to C in-so-far that it is fast, but only as fast as you
write functional (working) code. C can be fast just by slamming the keyboard.
Rust berates you like the barnicales on a rock as you keep passing `&'str` when
you need `String` and you have to add a vim motion to insert `.to_string()`
every five seconds... Sorry, where was I.

Ah, yes, C#. Distant cousins, I think. From keywords(`use` vs `using`) to
certain types (`String`). I haven't dove in far enough to find more, these are
just the one's I've noticed.

## So what are you saying?

If you're really struggling to figure out if you should give Rust a try,

<img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExMWR3NnFleXVlMDM5OGEzb3o3NWkyN3R5bDByaDRyY3VqazU3bDVzZCZlcD12MV9naWZzX3NlYXJjaCZjdD1n/GcSqyYa2aF8dy/giphy.gif" alt="JUST DO IT!" width=450/>

### Building

```sh
git clone https://github.com/wjorden/rust-book
cd rust-book
[ls to see the different projects]
cd <project>
cargo run
[help will automatically print]
```
