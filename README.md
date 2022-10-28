# todocli

A simple commandline tool i wrote using Rust.
The reason i post this on GitHub is that ive ran in to multiple very hard problems. Some of these problems were more or less undocumented.
I hope this helps you with learning rust and maybe building your own 'todocli'.

you can use this program as it is by running 'todocli list show' twice. once to generate a config file and data file in 
the default config location. (see bottom of this readme). then you can call 'todocli list add "Something that needs to get done"'
and it will add it to the data file. you can then see the data file by just calling the list show again.

you can mark todo's as completed in your list by calling 'todocli list done 0' to mark the todo at index 0.

if you have any questions about the code or about how to use the program, you can create an issue.

Here are the commands you need if you decide to clone it:

```bash
# First build the executable
$ cargo build

# Then run the created file by running one of these:

# Example for viewing the list
$ cargo run -- list show 
# OR
$ ./target/debug/todocli list show
```


## default config stored in: 

Linux:   /home/you/.config/todocli

Windows: C:\Users\you\AppData\Roaming\meloendev\todocli

macOS:   /Users/you/Library/Application Support/dev.meloendev.todocli
