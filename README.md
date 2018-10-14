# Visual Includes

A tool to help visualize C++ include trees.

The idea is to compile a C or C++ source file with clang or gcc and use the `-H`
option. The `-E` flag is also useful to prevent to compiler from doing anything
more than printing the header tree. With these options, the compiler prints the
include tree to stderr but isn't really easy to read as soon as they get
slightly complex which is nearly always the case. A sample output from including
`stdio.h`:

```
. /usr/include/stdio.h
.. /usr/include/bits/libc-header-start.h
... /usr/include/features.h
.... /usr/include/sys/cdefs.h
..... /usr/include/bits/wordsize.h
..... /usr/include/bits/long-double.h
.... /usr/include/gnu/stubs.h
..... /usr/include/gnu/stubs-64.h
.. /usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/stddef.h
.. /usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/stdarg.h
.. /usr/include/bits/types.h
... /usr/include/bits/wordsize.h
... /usr/include/bits/typesizes.h
.. /usr/include/bits/types/__fpos_t.h
... /usr/include/bits/types/__mbstate_t.h
.. /usr/include/bits/types/__fpos64_t.h
.. /usr/include/bits/types/__FILE.h
.. /usr/include/bits/types/FILE.h
.. /usr/include/bits/types/struct_FILE.h
.. /usr/include/bits/stdio_lim.h
.. /usr/include/bits/sys_errlist.h
```

The tool allows you to visualize this graph in an interactive TUI which does a
few things:

- you see how many headers are included by each given header,
- you can expand and collapse header nodes.

A few keys are used to control the output:

| Key | Use           |
| -   | -             |
| ↑   | move up       |
| ↓   | move down     |
| →   | expand node   |
| ←   | collapse node |
| [   | collapse all  |
| ]   | expand all    |

The same sample as above in our application with everything expanded:

```
* /usr/include/stdio.h (21)
    /usr/include/bits/libc-header-start.h (7)
      /usr/include/features.h (6)
        /usr/include/sys/cdefs.h (3)
          /usr/include/bits/wordsize.h (1)
          /usr/include/bits/long-double.h (1)
        /usr/include/gnu/stubs.h (2)
          /usr/include/gnu/stubs-64.h (1)
    /usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/stddef.h (1)
    /usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/stdarg.h (1)
    /usr/include/bits/types.h (3)
      /usr/include/bits/wordsize.h (1)
      /usr/include/bits/typesizes.h (1)
    /usr/include/bits/types/__fpos_t.h (2)
      /usr/include/bits/types/__mbstate_t.h (1)
    /usr/include/bits/types/__fpos64_t.h (1)
    /usr/include/bits/types/__FILE.h (1)
    /usr/include/bits/types/FILE.h (1)
    /usr/include/bits/types/struct_FILE.h (1)
    /usr/include/bits/stdio_lim.h (1)
    /usr/include/bits/sys_errlist.h (1)
```

# Known Issues

This is an ultra α release and most things still don't work:

- no scrolling,
- can't output graphviz files.
