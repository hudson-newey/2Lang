# 2 Lang

The worlds most efficient systems programming language

## Reserved Symbols & Words

### Comments

```C
// this is a comment and will be ignored
```

### Pre-Processor Macros

```C
#macroKey value
```

Is a macro. At compile time, it will replace all instances of #key with the `value`.

You can also define interpolated functions in this manner as all arguments will be accessible via the `$` symbol

e.g.

```c
#printNewLine PRINT "$1\n"
```

### Imports

```sh
@/path/fileName.2
```

Will import a file to be used

By using currying, you are able to create functions which take two arguments.

### Pre-Processor Code Execution

You can run code at compile time using the `@{}` syntax

It is best to explain this with "C-Like" syntax

For example, if I wanted to create macros for all the printable ASCII characters (A-Z, a-z), I could write a pre-processor code execution block like the following.

```c
// this std lib library does not currently exist
// but I do have plans to create 2lang C compiler macros
@/lib/clang/clang.2
@/lib/types/in8.2

@{
    #include <stdio.h>

    for (int i = 65; i < 122; i++) {
        printf("#%c %i\n", i);
    }
}
```

Note that the example above would not work as pre-processor block above would not work because pre-processor blocks can only run 2lang code, and not C-code. However, for this example, I have used C for clarity.

## Standard Library (`@lib/std.2`) Symbols & Words

```sh
PRINT "message"
```

The print keyword is used to print out text to the screen (planned; not currently functional)

---

```sh
;
```

The semi-column is an execute command. It will execute all commands in the CPU registers.

## Conventions

As there is currently no syntax highlighting, we write in all capital letters (similar to SQL)

## Examples

Hello World program

```C
@lib/std.2
PRINT "Hello World!"
```

---

Cheat Hello World program

```C
@lib/helloWorld.2
HELLO_WORLD
```

---

Localized macro example

```C
#MY_MACRO 11011001
MY_MACRO
```

will produce

```C
11011001
```

---

Macro with parameter example

```C
#USE_PARAMETER 0000 $
USE_PARAMETER 1111
```

should replace _$_ with 1111

## Command Line Options

## Compiler Options

- `-p`, `--preserve-intermediate` preserve the intermediary output produced by the pre-processor
- `-b`, `--generate-intermediate` takes an intermediary output (_.bin_) file produced by the pre-processor
- `-d`, `--debug` Run in debug mode (log file reads, etc...)
- `-s`, `--stdout` Output the final file contents to stdout

### Pre-Processor Options

- `--no-expand-strings` Does not expand strings to their binary representation
