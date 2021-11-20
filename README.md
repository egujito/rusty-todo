# rusty-todo

I wanted to remake a project I had done that used `ncurses.h` in C, the code was messy and I didn't have the patience to refactor it. So why not do rewrite it in rust as my first project?

(not finished yet)

# compiling and running

Compile:

```
$ cargo build
```

Compile & run:

```
$ cargo run
```

# Guide

### Managing tasks:

Move upwards: <kbd>k</kbd>

Move downwards: <kbd>j</kbd>

New task: <kbd>i</kbd>

Edit task: <kbd>e</kbd>

Delete current task: <kbd>d</kbd>

Toggle task state: <kbd>Enter</kbd>

Quit with exit code 0 (automatically saves state): <kbd>q</kbd>

### Inserting/editing tasks:

Discard the buffer: <kbd>Escape</kbd>

Confirm addition/edition of task: <kbd>Enter</kbd>

Cursor to the left: <kbd>Left Arrow Key</kbd>

Cursor to the right: <kbd>Right Arrow Key</kbd>

Delete a character from the buffer: <kbd>Backspace</kbd>


# Todo

> Implement the way of dragging tasks up or down the list

