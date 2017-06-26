# lines

> It's like cat, but better!

`lines` takes a file name and the range of lines (inclusive) and prints that
section of the file to standard output. Useful if you want to pipe to your
clipboard, or something like that.

Yes. I know I could have `sed -n` it, but hey, everyone should learn `rust`,
right?

Added options include displaying the line number, and some customizations on how
the line number is displayed.

# Usage

```
Usage: lines [options] <file> <start> <stop>
       lines (--help | --version)

Description:
    Prints only said section from the file, ranging [start,stop] (inclusive at
    both ends).

Options:
    -h, --help      Display this help and exits
    --version       Display version information
    -l, --lines     Display corresponding line number
    -s, --spaces S  Padding for line number
```
