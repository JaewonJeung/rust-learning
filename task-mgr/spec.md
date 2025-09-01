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
    - pretty print
- edit tasks

### v2
- list sort by different fields
- filter by priority or status
- attempt delete by label 


## Non func
- optimize edit and delete (HM)
- logging
- unittests
- config file/env variable reading
- use traits to make it hexagonal
    - fs save (json, sqlite)

- gpt review

- Advanced error handling
    - anyhow for internal error handling?
    - thiserror for custom error (maybe focus more on this)