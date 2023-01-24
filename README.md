# Universal Bookmark Manager

A [suckless](https://suckless.org/philosophy/) bookmark manager that is minimalistic, works offline and
can store whatever URI you want.

## Configuration

Copy the bookmark script and configure the variables in the config section.
You may use a different menu than [tofi](https://github.com/philj56/tofi),
like [dmenu](https://tools.suckless.org/dmenu/).

Bind the bookmark script to some key combination. With [i3](https://i3wm.org/) or [sway](https://swaywm.org/)
it is done like this:
```
# Pick bookmark
bindsym $mod+i exec bookmark pick
# Add bookmark
bindsym $mod+Shift+i exec bookmark add
# Remove bookmark
bindsym $mod+Control+Shift+i exec bookmark remove
```

## Usage

When you add a bookmark, the content of the primary clipboard is used (the selected text, not the copied one).
The menu then asks you for tags that you may want to bind to the new bookmark.
A list of already existing tags is shown, so that you avoid creating tags that are conceptually the same
but spelled in a slightly different way.
To stop adding tags, close the menu (usually with the `ESC` key).

Picking a bookmark is straightforward. The URI of the selected bookmark is copied to the secondary clipboard
(as if you copied it with `Ctrl+C`).
The URI is not written out using some automation tool like `ydotools`, because of security
concerns arising from giving access to `/dev/uinput`.

## Notifications

Notifications are sent using the `notify-send` command. If you do not receive any,
make sure to have a notification server like [dunst](https://dunst-project.org/) installed.

## Importing bookmarks from your browser

This repo contains a simple conversion software to import the bookmarks from a browser.
Consult [its readme](https://github.com/devgioele/universal-bookmark-manager/blob/main/netscape-to-universal/README.md) for further instructions.

## Design

### Storing bookmarks

Bookmarks are saved in a single plain text file and are organized with tags instead of directories.
This allows bookmarks to be bound to multiple keywords,
better suiting the network-like architecture of the human brain.
With directories, a bookmark would not be able to have multiple parent directories.

Example bookmark file:
```
https://github.com #github #programming
https://example.com #Something #example-tag #programming
```

### Organizing bookmarks

Humans tend to organize ideas in a network-like
fashion. Tags can represent networks, while directories cannot,
because a file cannot have multiple parent directories.
This is why bookmarks are organized with tags rather than with directories.
