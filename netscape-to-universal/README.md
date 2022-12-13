# netscape-to-universal

A simple conversion software to import your bookmarks from a Netscape HTML file.
The target format is the one used by the universal bookmark manager.

## Usage

1. Use your browser to export the bookmarks as HTML.
2. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and run `cargo install netscape-to-universal`.
3. Run `netscape-to-universal <input_file> >> <output_file>`, where `<input_file>` is the path to the HTML file
and `<output_file>` is the path to the bookmarks file (usually located at ~/.local/share/bookmarking/bookmarks).
4. Review the bookmarks file and edit any tags that you find inappropriate.
5. Optionally, remove the conversion software with `cargo uninstall netscape-to-universal` once you are done.

For a detailed description of the file formats, see [the specification](https://github.com/devgioele/universal-bookmark-manager/blob/main/formats.md).
