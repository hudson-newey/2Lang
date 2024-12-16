# DocGen Specification

2Lang has built-in support for automatic documentation generation.

It is heavily inspired by Java Doc and JSDoc.

To create a DocGen comment, you will need to use a block comment with two
starting asterisk's.

```ts
/** Your DocGen comment here */
```

## Keywords

### @description

```ts
/**
 * @description
 * Add a simple description of your macro
 */
#(MY::MACRO) (STD::PRINT) "Hello, World!"
```

### @param

```ts
/**
 * @param name
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @return

```ts
/**
 * @return An int32 value
 */
#(MY::MACRO) 42
```

### @example

```ts
/**
 * @example
 * (MY::MACRO) "World"
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @see

```ts
/**
 * @see https://wwww.google.com
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @since

```ts
/**
 * @since 1.0.0
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @deprecated

```ts
/**
 * @deprecated
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @author

```ts
/**
 * @author John Doe
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @version

```ts
/**
 * @version 1.0.0
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```

### @license

```ts
/**
 * @license MIT
 */
#(MY::MACRO) (STD::PRINT) "Hello, $!"
```