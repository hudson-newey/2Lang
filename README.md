# 2 Lang

The worlds most efficient systems programming language

## Reserved Symbols & Words

### Comments

```C
// this is a comment and will be ignored

/*
    This is a block comment and will be ignored
    it can span multiple lines
*/
```

### Doc-gen Comments

Doc comments can be used to automatically generate documentation for a file
or a set of files.

The format is heavily inspired by Java Doc & JavaScript's JSDoc

```ts
/**
 * @summary
 * This is a short description of a macro / function
 *
 * @description
 * This is a longer description of the macro / function
 * it can be multiple lines long
 *
 * @author First Last
 * @version 1.0.0
 *
 * @since 1.0.0
 * @see 2lang.awt
 *
 * @deprecated As of version 1.0
 *
 * @param fn An inner function to be executed
 * @param converter A converter function that will be applied to the output
 *
 * @returns A new value
 */
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

By using currying, you are able to create functions which take two arguments.

### Imports

```sh
@/path/fileName.2
```

Will import a file to be used

### Pre-Processor Code Execution

You can run code at compile time using the `@{}` syntax

It is best to explain this with "C-Like" syntax

For example, if I wanted to create macros for all the printable ASCII characters (A-Z, a-z), I could write a pre-processor code execution block like the following.

```c
// this std lib library does not currently exist
// but I do have plans to create 2lang C compiler macros
@/lib/clang/clang.2
@/lib/types/int8.2

@{
    #include <stdio.h>

    for (int i = 65; i < 122; i++) {
        printf("#%c %i\n", i);
    }
}
```

Note that the example above would not work as pre-processor block above would not work because pre-processor blocks can only run 2lang code, and not C-code. However, for this example, I have used C for clarity.

### Pre-Processor Code Execution (Using Shell STDOUT)

Alternatively, if you (understandably) don't want to create binary macros
for the pre-processor, it is possible to use shebangs to specify an interpreter.

Any output captured in STDOUT will be interpolated into the pre-processors
source input.

```python
@!/usr/bin/env python3{
    for i in range(65, 122):
        ascii_character = char(i)
        print(f"#${ascii_character} {i}")
}
```

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

- `--run` automatically runs the program after compilation

### Compiler Options

- `-b`, `--generate-intermediate` takes an intermediary output (_.bin_) file produced by the pre-processor
- `-d`, `--debug` Run in debug mode (log file reads, etc...)
- `-s`, `--stdout` Output the final file contents to stdout

### Pre-Processor Options

- `-p`, `--preserve-intermediate` preserve the output produced by the pre-processor
- `--preserve-linked` Preserves the statically linked file (without macros expanded)
- `--no-expand-strings` Does not expand strings to their binary representation

### Optimizer Options

- `--skip-optimizer` Does not run the optimizer on the output source code
