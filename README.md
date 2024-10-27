# Skip Icloud Sync

A Rust utility to prevent iCloud from syncing specific folders, particularly useful for large directories like `node_modules`.

## Features

- Creates a `.nosync` folder and symlinks it to prevent iCloud syncing
- Automatically adds the folder to `.gitignore` (optional)
- Supports custom folder names (default is `node_modules`)
- Checks if the current directory is within iCloud
- Option to skip adding the folder to `.gitignore`.

## Installation

1. Download prebuild binary from [Actions](https://github.com/da-vinci-noob/skip-icloud-sync/actions) Tab.

## Usage Examples

1. Default usage (for `node_modules`):
  ```skip-icloud-sync```
2. Specify a custom folder:
  ```skip-icloud-sync -f my_large_folder```
3. Skip adding to `.gitignore`:
  ```skip-icloud-sync -s```

## Building from Source

1. Ensure you have Rust installed
2. Clone the repository
3. Run ```cargo build --release```
4. The binary will be available in `target/release/skip-icloud-sync`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
