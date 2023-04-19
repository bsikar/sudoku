# Sudoku Solver with Real-Time Backtracking
This project is a Sudoku solver implemented using backtracking algorithm. The solver provides a GUI (Graphical User Interface) that displays the Sudoku board and fills in the missing cells with the correct numbers until the board is complete.

## Installation
Clone the repository and make sure that you have Rust programming language installed. Then, use the following command to run the project:
```bash
cargo run
```

## Usage
The solver is integrated with a graphical user interface that allows the user to interact with the board and see the solution in real-time. Once you run the program, you will see the Sudoku board displayed on the screen with some cells filled in with numbers. These represent the initial configuration of the board that needs to be solved.

The solver uses backtracking algorithm to find the solution. It iteratively fills in the empty cells with numbers and checks if the solution is valid. If it reaches a dead end, it backtracks and tries a different number until a valid solution is found.

The solver displays the solution in real-time on the screen. Once the board is complete, the solver stops and displays the final solution.

## Contributions
Contributions are welcome! Feel free to create a pull request or submit an issue if you encounter any problems or have any suggestions for improvement.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
