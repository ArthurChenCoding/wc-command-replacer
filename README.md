# wc command replacer
A command line tool to count the number of lines, words, and chars, in a given text file, similar to the unix wc command

## Build

```
cargo build --release
./target/release/text_file_analyzer <path_to_text_file>

```
Replace <path_to_text_file> with the path to the text file you want to analyze. The output will display the number of lines, words, and characters in the text file, followed by the file path.

This command-line tool reads a text file line by line, and for each line, it counts the number of characters and words (consecutive sequences of non-whitespace characters). Finally, it returns the total number of lines, words, and characters in the file.
