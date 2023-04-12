# tsp-solver
Rust program to efficiently solve the Traveling Salesperson Problem with branch and bound and heuristics. Solves the US Capitals graph in under 20 minutes on my local machine, and 12 minutes on my school's iLab machines. 

## Usage
The input is an n by n adjacency matrix entered in a file line by line, space separated
Run the script on "test.txt" for example with
```
cargo run --release test.txt
```

## Generating Test Case
You can use the provided python script generate_test.py to create a random euclidean test graph of dimension n with
```
python3 generate_test.py n
```
The graph will be in "test.txt". 