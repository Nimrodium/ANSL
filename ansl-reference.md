# sketch
```rs
  let var : u8 = 10;
  let var2 : u8 = var + 1;
  printf("var2 is %s",itoa(var2));
```

```rs
    fn main(){

    }

```

# Introduction
  ansl is a basic, high level language explicitely for the NISVC architecture
  ansl stands for advanced nisvc system language.
  its syntax is similar to rust, but the behavior more closely aligns to C.
  it is a very simple language, manual memory management, it gives nisvc a more human friendly syntax.

  the offical compiler compiles into nsm text files, which are then passed to the assembler.

  introduce variables using let
  introduce constant variables with const
  introduce static variables with static
  introduce functions with fn
  declare types with :
  scopes are defined with curly brackets
  arrays are defined with brackets
  tuples and functions use parentheseses

## preprocessor syntax

  ansl provides preprocessor instructions, the most useful being the include command, all preprocessor instructions like C start with #.
  s
  ### include
    the include command is the most useful preprocessor command. like C it can be used to include other files into the source of your program.
    files to be included can be located in a few different places, system installed libraries, project modules, or just about anywhere. in order to specify a file in the include command the location sub command must be included
      - system - looks in the system installed libraries repository (~/.ansl or /var/lib/ansl)
      - module - looks in the parent directory of the main file.
      - absolute - uses an absolute path provided
      this is in contrast to C which uses <> and "", which is ambigious
      ```
      #include system stdio
      #include module util
      ```
  # if else
    if, else, and fi can all be used for conditional code selection.



## Array



## Compiler Pipeline
ansl source code - lexed > tokenized source - parsed > AST - assembly resolution > logical assembly representation - rolling & spilling > virtual assembly - block stitching > assembly interal representation - compilation > nsm source code - assembler > machine code
