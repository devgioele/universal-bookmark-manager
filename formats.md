# Universal bookmark manager file format

A valid file is either empty or contains multiple lines, each one representing a bookmark entry.
The Backus-Naur form grammar for a valid line, assuming that the nonterminal symbol
`unicode-no-space` stands for any Unicode character that is not a space character, is this:

```
<line> ::= <text> <EOL> |
           <text> <tags> <EOL>

<text> ::= <unicode-no-space> |
           <unicode-no-space> <text>

<tags> ::= " #" <text> |
           " #" <text> <tags>
```

# Netscape bookmark file format

Not all browsers produce the same HTML file when exporting bookmarks.
However, the following elements are found coherently. Other elements are ignored.


The file starts with the following text:

```
<!DOCTYPE NETSCAPE-Bookmark-file-1>
    <!--This is an automatically generated file.
    It will be read and overwritten.
    Do Not Edit! -->
    <Title>Bookmarks</Title>
    <H1>Bookmarks</H1>
```

The rest of the file is as follows:
```
<DL>
{item}
{item}
{item}
.
.
.
</DL>
```

An item may be a subfolder, shortcut, feed, Web Slice, or icon.

If {item} refers to a subfolder, it is:
```
<DT><H3 FOLDED ADD_DATE="{date}">{title}</H3>
<DL><p>
    {item}
    {item}
    {item}
    .
    .
    .
</DL><p>
```
If {item} refers to a shortcut, it is:
```
<DT><A HREF="{url}" ADD_DATE="{date}" LAST_VISIT="{date}" LAST_MODIFIED="{date}">{title}</A>
```
