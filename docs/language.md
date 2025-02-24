# 2Lang Language Design

## Reserved Symbols & Words

I have tried to keep the number of symbols to a minimum so that you can build
you own language with a limited subset of reserved words.

- `//` Line comment
- `/*` Block comment start
- `*/` Block comment end
- `@` Pre-processor import
  - `@!` Pre-processor directive (intepretor) start
  - `@{` Pre-processor directive start
    - `}` Pre-processor directive end
- `#` Macro declaration
- `$` Macro declaration parameter

### Comments

```C
// this is a comment and will be ignored

/*
    This is a block comment and will be ignored
    it can span multiple lines
*/

// comments can also be in the middle of code
[CONST] = (TYPES::STRING) /** a basic message */ {"Hello world"};
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
