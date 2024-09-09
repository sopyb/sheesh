# sosh (Sopy Shell)

> [!NOTE]  
> If anyone actually uses this I promise to make a README that's not just bad jokes and self-deprecation.

Welcome to sosh, the shell that's doing its best impression of a real shell. Born from the wild idea of "What if I actually made a working shell? LOL", sosh is on a journey from "Hello, World!" to "Oh, it can has pipe".

## The "Why on Earth?" (Project Origin)

From being thoroughly burned out before the LLVM internship from previous, deeply unfulfilling, work. To the internship period struggling to learn how the LLVM project works. To the post-internship clarity (or madness, you be the judge) that fuels me to learn more about how everything I rely on works.

BAM! Like a segmentation fault while running your test suite, inspiration strikes! "What if I made a shell?" I thought, as if I had any idea what that entailed. And thus, sosh was born.

## Project Goals (or "What Was I Thinking?")

- Learn how shells work (because apparently, they don't run on magic)
- Create a Linux shell that doesn't immediately crash (rust moment)
- Achieve POSIX compliance (whatever that means - sounds fancy though!)
- Implement enough features so that switching from zsh doesn't make me feel like running `rm -rf /` every time I type a command

## The "What... why's this still missing" Roadmap

### Core Goals (The "Must-Haves" or else it's just a worse echo)
- [ ] ~~Reduce monster consumption~~ (Sleep is for the weak)
- [ ] Linux support (because who uses Windows?)
- [ ] POSIX compliance
- [ ] Customizable keybindings (so arch users can bind ctrl+j to `neofetch`, TOTALLY didn't search for a key that's not already bound in bash)

### Basic Functionality (AKA "The Bare Minimum to Call It a Shell")
- [ ] Command execution
- [ ] Command history (for the time you typed your password into the shell when you misspelled `sudo`)
- [ ] Built-in commands
- [ ] Tab autocomplete
- [ ] Redirection (`< > |`)
- [ ] Background processes (`fg`, `bg`) (for real, does anyone actually use those for multitasking?)
- [ ] Prompt customization

### Advanced Functionality (Or "It ain't so bad anymore")
- [ ] Tokenizer, Parser, and Executor
- [ ] Proper Environment Table tools (because I actually have ideas how to improve it, bet you'd love `console.table` in your shell)
- [ ] Script support
- [ ] Execution control (if, do while, while)
- [ ] Multiline command input (for those commands that can be qualified as short stories)

### Misc Functionality (The "Nice to Have" Pile)
- [ ] Alias support
- [ ] Syntax highlighting (making your commands look pretty)
- [ ] Config file
- [ ] Nix Home-Manager integration (I use NixOS BTW)

### The "In My Dreams" Features
- [ ] Windows support (some people just want to watch the world burn)
- [ ] MacOS support (I don't have a Mac, so this is scheduled for "Never")

## Current Status

sosh is currently in the "Hello, World!" phase. But don't worry, we'll be at "Accidental rm -rf /" in no time! I will patch it to add --no-preserve-root, I promise.

## Contributing

Feel free to contribute! Whether it's code, ideas, or just moral support as I question my life choices, all are welcome. Open an issue, or better yet, send a pack of monster.

## License

Distributed under the GPL-3.0 License. See `LICENSE` for more information.

## Contact

Sopy - contact@sopy.one

Project Link: [github.com/sopyb/sosh](https://github.com/sopyb/sosh)

Happy coding!

P.S. If this project inspires you to start your own "What was I thinking?" coding adventure, my work here is done. Now, back to figuring out how to make this shell work like a shell should. Who knows, maybe I make some blog posts out of this.
