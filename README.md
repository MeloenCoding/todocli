# todocli

A simple commandline tool i wrote using Rust, clap.
The reason i post this on GitHub is that ive ran in to multiple very hard problems. Some of these problems were more or less undocumented.
I hope this helps you with learning rust and maybe building your own 'todocli'.

You can't use it as it is because i linked it to my api for personal use. Altough you can simply fix this by changing the api_link
variable in main.rs to the file you want to store your todos.

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
