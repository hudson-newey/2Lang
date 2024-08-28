# Standard Library Templates

Standard library templates will be executed at their import/insertion location.

Whereas the rest of the standard library will attempt to not modify the
program until a macro declared inside the import is used.

## Why should I use templates

Templates reduce boilerplate such as setting up ELF headers.

However, where possible, you should attempt to use non-mutative imports instead.
