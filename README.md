TODO

Functionality:
- search for a <SOMETHING> in a <WHERE>

SOMETHING:
- string
- regex

WHERE:
- file(s)
- standard input
- directory

OUTPUT:
- lines containing the <SOMETHING>
- lines with line numbers
- file name + line number + line
- file name + line number + line + colorized <SOMETHING>
- file name + line number + line + colorized <SOMETHING> + context

FLAGS:
- color --color [HEX_COLOR]
- line number -l --line-number
- file name -h --file-name
- context -C --context [NUMBER]
- count -c --count


EXAMPLES:
- search for a string in a file:
  $ mgrep "string" file.txt

- search for a string in multiple files:
  $ mgrep "string" file1.txt file2.txt file3.txt

- search for a string in all files in a directory:
  $ mgrep "string" directory/

- search for a string in all files in a directory and its subdirectories:
  $ mgrep "string" directory/ -r

- search for a string in standard input:
  $ echo "string" | mgrep "string"

- search for a string in standard input and colorize the string:
  $ echo "string" | mgrep "string" --color

- search for a string in standard input and colorize the string and show context (in this case 2 lines before and 2 lines after):
  $ echo "string" | mgrep "string" --color --context 2

- search for a string in a file and show the line number:
  $ mgrep "string" file.txt --line-number

- search for a string in a file and show the file name:
  $ mgrep "string" file.txt --file-name


How to implement:
- read the arguments
- read the input
- search for the string in the input
- print the output

- read the arguments:
  RULES:
  - the first argument are the flags ()
  - the second argument are args with values ()
  - the third argument is pattern
  - the rest of the arguments are files
    - if no files are provided, read from standard input
    - if 1 file is provided, read from that file
    - if multiple files are provided, read from those files
