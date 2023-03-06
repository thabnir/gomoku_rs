# Gomoku AI in Rust

## Issues / Roadmap

### My Code

#### Figure out how I actually want my data structured

- I'm wasting a lot of times making structs and things that I don't even know the actual use of
- Need a board, something to keep track of the turn

#### Lookup / Transposition Tables

- Need a hashing algorithm

#### How do I best check for 5-in-a-row chains?

- Should I have one board per color?
  - Probably, struggling to think of why not, and it means I only have to check which type of stone a sequence is once.
  - I could also have 3 board representations. Just adds a little bit of space complexity, which shouldn't be a problem.
- Should I use bitmaps for && chaining of bits?
  - Should probalby just worry about this later if it becomes a bottleneck. Better to finish it but have it be maybe slow than worry about this endlessly

### Gomocup

- I have no idea how to convert the C++ `pisqpipe` code to Rust.
- I also don't know how to do that for the Java or C# versions of the code.
- Maybe that's a problem to solve AFTER I get my AI working, though?
  - Unclear; I don't want to discover I've been doing something that would totally disqualify me from the Gomocup too late.

#### Then again, do I even care about the Gomocup?

Kinda

- It's much cooler on the resume
- I can actually easily test my AI
  - I think this is reason enough
- It would be nice to not have to make the GUI myself
  - But it would also be nice to get the experience of making the GUI myself
  - Maybe I could make my own GUI but also interop with theirs?
