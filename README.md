# leesp
Lisp interpreter

----
Possible solutions for che cons cell:
. Cons(Rc<Cell>, Rc<Cell>), : everything is very clear and multiple references to the same cell are permitted, but very low performances. creating a new cons cell you must move
. Cons()