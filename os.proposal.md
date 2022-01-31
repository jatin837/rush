# OS MTE PROPOSAL

## JATIN PANDEY(__2K19/EE/125__) & SHIV KUMAR VERMA(__2K19/EE/233__)


<figure>![](dtu.logo.jpeg){ style="width: 10%; margin: auto;" }</figure>

# SYNOPSIS
**RuSh** is a __UNIX__ shell interpreter, writtern in **RUST** programming language. Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.

A shell does three main things in its lifetime.

- __Initialize__: In this step, a typical shell would read and execute its configuration files. These change aspects of the shell’s behavior.
- __Interpret__: Next, the shell reads commands from stdin (which could be interactive, or a file) and executes them.
- __Terminate__: After its commands are executed, the shell executes any shutdown commands, frees up any memory, and terminates.
The Goal of RuSh is to be a better and faster Unix shell runtime, unlike common shell interpreter such as __Bash__, __ksh__ or __sh__.

Common features RuSh provides are:

+ syntax highlighting
+ information about git repositories
+ better tab completions and suggestions
