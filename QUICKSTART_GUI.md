# Quick Start - Rust/egui GUI

## Building the GUI

### Prerequisites
- Rust 1.70+ (nightly as per rust-toolchain.toml)
- Cargo
- Python 3 (for encryption and signing scripts)
- Windows SDK or build tools for Windows targets

### Build Steps

1. **Compile the GUI binary:**
   ```bash
   cargo build --release --bin gui
   ```

2. **Run the GUI:**
   ```bash
   cargo run --release --bin gui
   ```

   Or run the built binary directly:
   ```bash
   ./target/release/gui.exe  # Windows
   ./target/release/gui      # Linux/macOS
   ```

## First-Time Setup

1. **Prepare Input Files:**
   - Place shellcode binaries in the `input/` directory
   - Place icon files in the `icons/` directory
   - Place signing templates in `sign/app/` directory (optional)

2. **Verify Configuration:**
   - Check `config/plugins.json` has your desired encryption/execution methods
   - The GUI will auto-load available options from this file

3. **Prepare Output Directory:**
   - The GUI will create `output/` directory automatically
   - Built executables will be saved here with timestamp-based names

## Using the GUI

### Step-by-Step Build Process

1. **Select Shellcode**
   - Click the "📁 Select" button next to "Shellcode"
   - Choose a `.bin` file from your `input/` directory

2. **Choose Encryption**
   - Select encryption algorithm (ipv4, ipv6, aes, etc.)
   - Select encoding method (base64, base32, or none)

3. **Pick Icon (Optional)**
   - Click "📁 Select" next to "Icon File"
   - Choose a `.ico` file, or leave default (excel.ico)

4. **Select Memory Mode**
   - Choose how shellcode memory is allocated
   - Options depend on your `config/plugins.json`

5. **Enable Detections (Optional)**
   - Check any sandbox/VM detection methods you want
   - These add evasion capabilities

6. **Choose Execution Method**
   - Select from available run modes
   - Some modes may require additional inputs:
     - **Pattern 2 Modes**: Enter target program path
     - **Pattern 3 Modes**: Enter target process ID

7. **Signature Forging (Optional)**
   - Click "📁 Select" to choose a donor EXE
   - Check "Enable Signature" to copy PE signature
   - Check "File Bundling" for resource forgery

8. **Build Configuration**
   - Select target architecture (x64, x86, ARM64, etc.)
   - Check "Win7 Compatibility" if needed

9. **Generate**
   - Click the "🚀 Generate" button
   - Watch the progress bar and build log
   - Output file will be saved in `output/` when complete

## Build Log

The GUI displays real-time output from:
- Encryption process (encrypt.py)
- Rust compilation (cargo build)
- Output file operations
- Signing process (if enabled)

Watch for any errors and check the log for troubleshooting.

## Build Targets

Available compilation targets:

| Target | Description |
|--------|-------------|
| x86_64-pc-windows-msvc | Windows 64-bit (MSVC, recommended) |
| i686-pc-windows-msvc | Windows 32-bit (MSVC) |
| x86_64-pc-windows-gnu | Windows 64-bit (MinGW) |
| i686-pc-windows-gnu | Windows 32-bit (MinGW) |
| aarch64-pc-windows-msvc | Windows ARM64 (experimental) |

## Troubleshooting

### "Failed to read config/plugins.json"
- Verify `config/plugins.json` exists
- Check file is valid JSON (use a JSON validator)
- Ensure it has required fields: encryption, run_modes

### "Build output not found"
- Cargo build may have failed
- Check build log for compilation errors
- Verify Rust compiler is properly installed
- Try running `cargo check --bin rsl` to verify compilation

### "No files in input directory"
- Create an `input/` directory if it doesn't exist
- Add `.bin` files to the directory
- Restart the GUI for it to scan for new files

### Build takes very long
- First build includes downloading and compiling dependencies
- Subsequent builds will be faster (using cache)
- Building for different targets requires separate compilations

### Python errors in log
- Ensure Python 3 is installed and in PATH
- Run `python3 --version` to verify
- Check `encrypt.py` exists in project root
- For signing: ensure `sign/sigthief.py` exists

## Output Files

- **Location**: `output/` directory
- **Naming**: Timestamp-based (e.g., `1a2b3c4d.exe`)
- **Format**: Standalone Windows executable
- **No Dependencies**: Output binary includes all encrypted payload

## Advanced Usage

### Command Line (Original Tool)

The original command-line tool is still available:

```bash
cargo build --release --bin rsl
./target/release/rsl.exe  # Windows
./target/release/rsl      # Linux/macOS
```

### Custom Configuration

Edit `config/plugins.json` to:
- Add new encryption methods
- Modify run modes
- Change default values
- Add new VM checks

The GUI will automatically detect changes on next startup.

### Features Reference

Build process uses Cargo features. The GUI automatically selects features based on your choices:

**Encryption**: decrypt_ipv4, decrypt_ipv6, decrypt_aes, decrypt_xchacha20, decrypt_rc4, decrypt_mac, decrypt_uuid

**Encoding**: base64_decode, base32_decode, none_decode

**Run Modes**: run_enum_ui, run_create_thread, run_gdi_families, run_early_bird_apc, run_create_remote_thread

**Memory**: alloc_mem_va, alloc_mem_global, alloc_mem_local, alloc_mem_heap, alloc_mem_mapview

**Detections**: vm_check_tick, vm_check_mouse, vm_check_desktop_files, vm_check_c_drive

**Other**: with_forgery, win7

## Help & Support

- Check `GUI_IMPLEMENTATION_SUMMARY.md` for architecture details
- Review `src/bin/README.md` for module documentation
- See `IMPLEMENTATION_CHECKLIST.md` for complete feature list
- Examine `config/plugins.json` for configuration syntax

## Next Steps

1. Build and run the GUI: `cargo run --release --bin gui`
2. Prepare your input files in the `input/` directory
3. Generate your first payload
4. Check `output/` for the compiled executable
5. Customize `config/plugins.json` for your needs

Enjoy using RSL Builder! 🚀
