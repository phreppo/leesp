# leesp

Lisp interpreter for the version 1.5 of the language described in [Lisp 1.5 Programmer's manual](http://www.softwarepreservation.org/projects/LISP/book/LISP%201.5%20Programmers%20Manual.pdf). This is a revisited full-functional Lisp implementation: this means that when a cell is used in a procedure it is cloned, and that the every times `Nil` appears a new instance of the cell is created. Functions are _first class objects_ and values are _immutables_.
The true peculiarity of this Lisp implementation is that has an *implicit garbage collector* produced by Rust's system for handling the memory. 

The built-in Lisp supported constructs (for now) are: 
* `CAR`
* `CDR`
* `CONS`
* `LAMBDA`
* `QUOTE`
* `COND`
* `ATOM`
* `EQ`
* `T`
* `LABEL`
* `+`
