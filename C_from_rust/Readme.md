

## BUILD LIBRARY
In order to build the C library run the following commands:

```
gcc -c c_stuff.c
gcc -o libstuff.so -shared c_stuff.o
````

## BUILD RUSTC

```
rustc main.rs -L ./
```

**NOTE:** This code should be build by just using ```cargo build``` but I still haven't manage to do it.
