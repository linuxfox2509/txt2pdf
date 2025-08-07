# txt2pdf

txt2pdf is a Rust application that converts plain text files into PDF documents. This project utilizes various libraries for file handling and PDF generation, providing a simple and efficient way to create PDFs from text.

## Features

- Convert text files (.txt) to PDF format (.pdf)
- Easy to use command-line interface
- Supports basic text formatting

## Installation

To build and run the application, ensure you have Rust and Cargo installed on your system. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Clone the repository:

```
git clone https://github.com/linuxfox2509/txt2pdf.git
cd txt2pdf
```

Build the project:

```
cargo build --release
```

## Usage

To convert a text file to PDF, run the following command:

```
./target/release/txt2pdf <input_file.txt> <output_file.pdf>
```

Replace `<input_file.txt>` with the path to your text file and `<output_file.pdf>` with the desired name for the output PDF file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
