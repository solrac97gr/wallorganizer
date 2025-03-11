# WallOrganizer 🖼️ - Organize your MAC wallpappers by creation date

WallOrganizer is a Rust-based application designed to organize and manage image files within a specified directory. It provides functionality to read files, identify image files, and rename them based on certain criteria.

## Features ✨

- **Read Files from Directory** 📂: Scans a specified directory and retrieves all file paths.
- **Image Identification** 🖼️: Checks if a file is an image based on its extension.
- **File Operation Check** 🔍: Determines if a file has been previously operated on by checking its filename.
- **File Renaming** 📝: Renames files based on their creation date and a specified format.

## Getting Started 🚀

### Prerequisites 📋

- Rust programming language installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).

### Installation 🛠️

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/wallorganizer.git
   cd wallorganizer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Usage 🏃

Run the application with the following command:

```bash
cargo run -- path/to/your/directory
```

Replace `path/to/your/directory` with the path to the directory you want to organize.

## Makefile Commands 📜

The project includes a `Makefile` to simplify common tasks. Here are the available commands:

- **Build the Project** 🔨: Compiles the project in release mode.
  ```bash
  make build
  ```

- **Run the Project** ▶️: Executes the application using the example directory.
  ```bash
  make run
  ```

- **Install the Application** 📥: Builds and installs the application to `/usr/local/bin/`.
  ```bash
  make install
  ```

- **Uninstall the Application** ❌: Removes the application from `/usr/local/bin/`.
  ```bash
  make uninstall
  ```

To use these commands, simply run `make <command>` in the terminal from the project's root directory.

## Contributing 🤝

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## Acknowledgments 🙏

- Rust community for their extensive documentation and support. 