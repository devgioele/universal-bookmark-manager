# Universal Bookmark Manager

A [suckless](https://suckless.org/philosophy/) bookmark manager that is minimalistic, works offline and
can store whatever URI you want.

## Usage

Copy the bookmark script and configure the variables in the config section.
You may use different menu than [tofi](https://github.com/philj56/tofi),
like [dmenu](https://tools.suckless.org/dmenu/).

## Importing bookmarks from your browser

This repo contains a small conversion software to import your bookmarks from a Netscape HTML file.
Here is how to do it:
1. Use your browser to export the bookmarks as HTML.
2. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and run `cargo install netscape-to-universal`.
3. Run `netscape-to-universal <input_file> >> <output_file>`, where `<input_file>` is the path to the HTML file
and `<output_file>` is the path to the bookmarks file (usually located at ~/.local/share/bookmarking/bookmarks).

## How bookmarks are stored

Bookmarks are saved in a single plain text file and are organized with tags instead of directories.
This allows bookmarks to be bound to multiple keywords,
better suiting the network-like architecture of the human brain.
With directories, a bookmark would not be able to have multiple parent directories.

Example bookmark file:
```
https://github.com #github #programming
https://example.com #Something #example-tag #programming
```
