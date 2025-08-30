Robust CLI Task Manager

CLI task manager
- JSON serialize and deserialize structs (serde_json?)
- File handling (file saves)
- rust anyhow
- clap for CLI interface
- logger

## Functional Req
- create tasks
    - label, description, priority, status
- delete tasks

- list tasks (sort and filter)
    - sort by different fields
    - filter by priority or status
- edit tasks

## Non func
- use traits to make it hexagonal
    - fs save
- logging
- config file/env variable reading for swappable file saving mechanism (json, sqlite)
- unittests
- Advanced error handling
    - anyhow for internal error handling?
    - thiserror for custom error (maybe focus more on this)