Week 1: The Robust CLI Task Manager

    •    Project: We'll build a command-line task manager that's more than just a toy. It will parse complex commands and persist its state to the disk as a JSON file.
    •    Themes & Concepts:
    ◦    Data Modeling: Using structs to represent tasks (e.g., with an ID, description, and status).
    ◦    Information Packing (serde_json): Serializing your list of task structs into a JSON file and deserializing it back when the program starts.
    ◦    File Handling & System I/O: Reading from and writing to the file system (std::fs). This is a foundational system call interaction.
    ◦    Advanced Argument Parsing: Using a powerful crate like clap to create a professional CLI interface (e.g., task-manager add "buy milk" --priority high).
    ◦    Logging: Using the log and env_logger crates to add different levels of logging (info, warn, error) to see what your application is doing internally.

Weeks 2-4: Build Your Own Redis (A Multi-Week Epic)

This is our centerpiece project. We'll build a simplified, in-memory key-value store that speaks the Redis protocol.
    •    
Week 2: Part 1 - The TCP Foundation

    ◦    Project: Write a basic, single-threaded TCP server that can accept a connection and echo back whatever the client sends.
    ◦    Themes & Concepts:
    ▪    Low-Level Networking: Working directly with TCP listeners and streams from Rust's standard library or a framework like tokio.
    ▪    Intro to Async: We'll introduce the async/.await syntax, which is essential for modern network programming in Rust.
    •    
Week 3: Part 2 - The Redis Protocol (RESP)

    ◦    Project: Evolve the echo server into a proper key-value store. It will now parse the Redis protocol (RESP) and handle basic commands like PING, SET, and GET.
    ◦    Themes & Concepts:
    ▪    Protocol Parsing: A deep dive into information packing/unpacking by manually parsing a binary protocol. This involves careful byte manipulation and error handling.
    ▪    In-Memory Storage: Using a HashMap as our core data store.
    •    
Week 4: Part 3 - Concurrent Clients

    ◦    Project: Make our Redis server capable of handling many clients at the same time.
    ◦    Themes & Concepts:
    ▪    Multithreading/Async: This is the core of this week. We'll use tokio's task spawning to handle each client connection in its own asynchronous task.
    ▪    State Management: Learning to share state (our HashMap data store) safely across multiple threads/tasks using tools like Arc and Mutex.

Week 5: File Compressor with FFI

    •    Project: Let's create a command-line tool that can compress and decompress files using a battle-tested C library. Instead of writing a new C project, we'll link against an existing, widely-used one: zlib.
    •    Themes & Concepts:
    ◦    Foreign Function Interface (FFI): Writing the Rust "bindings" or declarations that allow us to safely call functions from the zlib shared library.
    ◦    Unsafe Rust: FFI requires dipping our toes into the unsafe keyword. We'll learn why it's needed and how to use it responsibly to create a safe abstraction.
    ◦    System Calls: The zlib library will be making the underlying system calls for file I/O, and we'll be orchestrating it from Rust.

Week 6: The API Weather Client

    •    Project: A command-line tool that takes a city name and fetches the current weather from a public API, like the OpenWeatherMap API.
    •    Themes & Concepts:
    ◦    High-Level Networking (HTTPS): Using a high-level crate like reqwest to make robust, simple HTTP GET requests.
    ◦    rustls Usage: We can explicitly configure reqwest to use rustls as its TLS backend, giving you direct experience with it.
    ◦    API Interaction: Working with API keys, query parameters, and parsing a complex, nested JSON response into clean Rust structs (revisiting our serde skills from Week 1).

Week 7: Profiling and Optimizing Redis

    •    Project: We'll return to our Redis server from Week 4 and put on our performance engineering hats.
    •    Themes & Concepts:
    ◦    Performance Measurement: Using benchmarking libraries like criterion to get reliable performance numbers for our server's command throughput.
    ◦    Profiling: Using tools like cargo-flamegraph and perf to create a visual profile of our application, showing exactly where it's spending its time.
    ◦    Optimization: Based on the profiling data, we'll identify bottlenecks (Is it lock contention? Memory allocation? Inefficient parsing?) and apply optimizations to make our server faster.
