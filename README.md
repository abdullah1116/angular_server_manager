# Program: Angular Server Error Restart

This is a Rust program that monitors the output of an Angular server for a specific error message and restarts the server if the error is detected. The error message that triggers the restart is customizable.
How It Works

The program runs a provided command (e.g., starting an Angular server) in a loop. It continuously reads the output from the command and looks for a specific error message. If the error message is detected in the output, it kills the running process and restarts it.
Usage

```bash
    # Compile the program
    $ cargo build

    # Run the program with your desired command
    $ cargo run ng serve

```

Replace <your_command_here> with the command that starts your Angular server.

### Dependencies

This program relies on the following Rust dependencies:

    tokio: Asynchronous runtime for Rust.
    std::env: Rust's standard library for environment variables.
    std::time::Duration: Rust's standard library for handling time durations.

### Important Notes

    The provided command is expected to start and run the Angular server.
    The program will keep running in a loop, checking for the error message and restarting the server as needed.

## Example

Here's how you can use this program to restart an Angular server when it encounters a specific error message:

```bash
$ cargo run ng serve
```

Replace ng serve with your own Angular server start command.
License

This program is provided as-is under the MIT License. You can find the full license text in the `LICENSE` file.

Feel free to modify and use this program to fit your needs.
