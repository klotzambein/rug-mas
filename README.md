# Multi agent systems - Group D16 - Gossip markets
This repository contains our implementation of a multi agent simulation of
financial markets based on the papers listed below. Additionally, we implemented
a few features on top.

## Papers
* [Directed Gossip Algorithms](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.314.849&rep=rep1&type=pdf)
* [Agent-based simulation of a financial market](https://arxiv.org/pdf/cond-mat/0103600.pdf)

## Running
To run our simulation compile the code using cargo and run it:
```sh
cargo run --release -- --help
```
This will print all the command line options. To visualize a simulation with 1000 steps and the combined config run this
command:
```sh
cargo run --release -- run -n 1000 -c configs/combined.toml -w
```

Finally, to generate all the plots used in the report run the file
`./plot.fish`. This script depends on python3 and fish as well as a unix
environment.