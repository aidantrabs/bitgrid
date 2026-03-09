# bitgrid

a project exploring the mathematics behind elementary cellular automata and emergent complexity.

## what is this?

bitgrid implements elementary cellular automata - the simplest class of one-dimensional cellular automata studied by stephen wolfram. a single byte (0-255) encodes a complete rule that determines how a row of binary cells evolves over time.

from these trivial local rules, complex global behavior emerges - chaos, fractals, and even turing completeness.

## rules we study

- **rule 30** - chaotic, aperiodic behavior from a deterministic system
- **rule 90** - produces the sierpinski triangle, a self-similar fractal equivalent to xor(left, right)
- **rule 110** - proven turing complete by matthew cook in 2004

## usage

```bash
# run a rule in the terminal
cargo run -- 30

# specify width and generations
cargo run -- 90 201 100

# show population density per generation
cargo run -- 30 81 40 --density

# show shannon entropy per generation
cargo run -- 110 81 40 --entropy

# combine flags
cargo run -- 30 81 40 --density --entropy

# save as png image
cargo run -- 90 201 100 --png

# compare multiple rules
cargo run -- --compare
```

## project structure

```
src/
  rule.rs        - rule encoding and lookup
  automaton.rs   - grid state and evolution
  analysis.rs    - density and entropy metrics
  experiment.rs  - multi-rule comparison
  render.rs      - png image output
  main.rs        - cli interface
```

## the math

there are exactly 256 elementary cellular automata. each rule is a function from 3-bit neighborhoods to a single output bit:

```
f: {0,1}^3 -> {0,1}
```

since there are 2^3 = 8 possible neighborhoods and each maps to 0 or 1, the total number of rules is 2^8 = 256. the rule number is the decimal value of the 8-bit output table.

to apply a rule:

```
index = left * 4 + center * 2 + right
output = (rule >> index) & 1
```

## license

mit
