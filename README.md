# 2 Lang

The worlds most efficient systems programming language

## Reserved Symbols & Words

```C
// this is a comment and will be ignored
```

---

```C
#macroKey value
```

Is a macro. At compile time, it will replace all instances of #key with the `value`.

You can also define interpolated functions in this manner as all arguments will be accessible via the `$` symbol

e.g.

```C
#printNewLine PRINT "$1\n"
```

---

```
@/path/fileName.2
```

Will import a file to be used

By using currying, you are able to create functions which take two arguments.

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
@lib/std.2
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

## Compiler

Preserve the intermediary binary output

```sh
cargo r <filePath> -p
```

---

Take in an intermediary binary (_.bin_ file)

```sh
cargo r <filePath> -b
```
