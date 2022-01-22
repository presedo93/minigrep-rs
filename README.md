# Minigrep Rust

An example taken from the Rust Programming Book on how to work with modules and features of Rust. The aim of this project is to search in a text file for lines containing the word given as argument.

## Some extras...
Apart of using the `colorized` crate (just to have some colors). The repo also has a **Dockerfile**. It supports multi-stage build, so the code will be compiled in the Rust container and the executable run in an Alpine.

    docker build -t minigrep .

The final image size is about 9.5MB. To deal with alpine's `musl`, the build target is changed. So, to run the final container with a file:

    docker run -it -v ${PWD}/poem.txt:/poem.txt minigrep ./minigrep to /poem.txt