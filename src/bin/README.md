# RSL GUI - Rust/egui Implementation

This directory contains the Rust-based graphical user interface for the RSL (RustSL) builder, replacing the original PyQt5 implementation.

## Architecture

The GUI is structured into modular components for maintainability and clarity:

### Files

- **gui.rs** - Entry point for the GUI application. Initializes egui/eframe and starts the main event loop.

- **app.rs** - Main application state and UI logic. Contains:
  - `RSLApp` struct managing all GUI state (file selections, combo box indices, checkbox states)
  - `update()` method implementing the egui UI rendering
  - Parameter collection logic
  - Build workflow coordination
  - Message processing from background worker threads

- **config.rs** - Configuration management module:
  - Loads and parses `config/plugins.json`
  - Provides methods to build feature/ID mappings
  - Handles default value retrieval
  - Supports all configuration types: encryption, memory modes, run modes, VM checks

- **worker.rs** - Background task execution:
  - `BuildWorker` handles the build pipeline in a separate thread
  - `BuildParams` struct carries all user-selected options
  - Orchestrates the workflow: encryption → build → output copy → optional signing
  - Uses `std::sync::mpsc` channels for thread communication
  - Spawns external Python scripts (`encrypt.py`, `sigthief.py`) as subprocesses
  - Reports progress and log messages back to UI

- **ui.rs** - UI state helper types (for future expansion):
  - `ComboBoxState` - Manages dropdown selections
  - `CheckboxGridState` - Manages checkbox groups
  - `FilePickerState` - Manages file selections

## Features

### UI Components

1. **Shellcode Input** - Select .bin files from input directory
2. **Encryption Method** - Choose encryption and encoding algorithms
3. **Icon Selection** - Pick custom icon or use defaults
4. **Memory Allocation** - Select memory allocation strategy
5. **Sandbox Detection** - Enable/disable VM/sandbox detection features
6. **Run Mode** - Choose execution method with conditional inputs for target program or PID
7. **Signature Forgery** - Optional signature copying and file bundling
8. **Build Configuration** - Select target platform and Win7 compatibility
9. **Progress Tracking** - Visual progress bar during build
10. **Build Log** - Real-time output from build process

### Background Tasks

The GUI uses standard Rust threading (`std::thread`) rather than async:
- Worker thread spawned when user clicks "Generate"
- Messages sent back to UI via `mpsc::Receiver`
- Non-blocking UI updates with `try_recv()`
- UI remains responsive during long operations

### File Operations

- **File Dialogs** - Uses `rfd` crate for native file picker dialogs
- **Payload Encryption** - Calls Python's `encrypt.py` with appropriate parameters
- **Binary Compilation** - Spawns `cargo` with selected features and target
- **Output Management** - Copies and renames compiled executable with timestamp-based naming
- **Signature Operations** - Optional call to `sign/sigthief.py` for PE signature cloning

## Configuration

The GUI reads `config/plugins.json` to dynamically populate:
- Available encryption methods
- Memory allocation modes
- Run/injection modes
- Sandbox detection techniques
- Default values for each category

This allows the UI to adapt without code changes when the configuration is updated.

## Build and Run

```bash
# Build the GUI binary
cargo build --release --bin gui

# Run the GUI application
cargo run --release --bin gui

# The original CLI tool remains available
cargo build --release --bin rsl
```

## Implementation Details

### State Management

The `RSLApp` struct maintains all UI state as simple types:
- `Vec<(String, String)>` for combo box items (label, id pairs)
- `usize` for selected indices
- `Option<PathBuf>` for file selections
- `Vec<(String, String, bool)>` for checkboxes (id, label, checked)
- `String` for text inputs and log output

### Message Flow

1. **User Interaction** → Updates app state
2. **Generate Button** → Collects `BuildParams` and spawns worker thread
3. **Worker Thread** → Executes build pipeline, sends `WorkerMessage` updates
4. **UI Update Loop** → `process_worker_messages()` reads from channel, updates state
5. **egui Render** → Displays updated state

### Python Integration

External Python scripts are called as subprocesses:
- `encrypt.py` - Encryption and encoding of shellcode
- `sign/sigthief.py` - PE signature copying from donor executable

Results are captured via stdout/stderr and displayed in build log.

## Design Decisions

1. **Threading Model** - Uses `std::thread` instead of async for simplicity. Adequate for this use case where operations are typically I/O bound (subprocess spawning, file I/O).

2. **State Storage** - Keeps UI state simple as basic Rust types rather than complex enums, making it easier to reason about state transitions.

3. **Config Format** - Continues using JSON (plugins.json) for compatibility and human readability.

4. **File Dialogs** - Uses `rfd` crate for native file dialogs on each platform.

5. **Message Passing** - Uses `std::sync::mpsc` (multi-producer, single-consumer) channels for thread communication, providing backpressure and thread safety.

## Future Enhancements

- Add progress bar for individual build stages
- Implement cancel/stop build functionality
- Add build profile selection (debug/release optimization levels)
- Implement project save/load for build configurations
- Add build history/recent builds tracking
- Implement dark mode theme support
