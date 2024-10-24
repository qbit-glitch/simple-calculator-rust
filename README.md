# Simple Calculator in Rust

This project is a simple command-line calculator implemented in Rust. It allows users to perform basic arithmetic operations such as addition, subtraction, multiplication, and division on two numbers.

## Features

- Perform basic arithmetic operations: addition, subtraction, multiplication, and division.
- Handle user input for two numbers and an operation.
- Error handling for invalid inputs and division by zero.

## Requirements

- [Rust](https://www.rust-lang.org/) installed on your system.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/qbit-glitch/simple-calculator-rust.git
   cd simple-calculator-rust
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Usage

When you run the program, you will be prompted to enter two numbers and an arithmetic operation in the following format:

```
<num1> <num2> <operation>
```

### Example Inputs

- To add two numbers:
  ```
  5 3 add
  ```
  Output:
  ```
  Addition Result: 8
  ```

- To subtract two numbers:
  ```
  10 4 subtract
  ```
  Output:
  ```
  Subtraction Result: 6
  ```

- To multiply two numbers:
  ```
  7 8 multiply
  ```
  Output:
  ```
  Multiplication Result: 56
  ```

- To divide two numbers:
  ```
  20 4 divide
  ```
  Output:
  ```
  Division Result: 5
  ```

### Error Handling

The program handles various errors, such as:

- Invalid number of inputs:
  ```
  Invalid number of inputs
  ```

- Invalid number entries:
  ```
  Error: Not entered 1st number
  ```

- Invalid operator:
  ```
  Invalid operator
  ```

- Division by zero:
  ```
  Error: Division by zero!
  ```

## Contributing

Contributions are welcome! Feel free to submit a pull request or create an issue for any enhancements or bugs you encounter.



