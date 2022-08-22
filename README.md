# `carcer` (CAR Cdr dEsugaRer)

`carcer` is a silly expander for combinations of `car` and `cdr` operations.

```lisp
$ carcer "(car x)"
"(car x)" is equivalent to "(car x)"
$ carcer "(cadr x)"
"(cadr y)" is equivalent to "(car (cdr y))"
$ carcer "(cddadr x)"
"(cddadr y)" is equivalent to "(cdr (cdr (car (cdr y))))"
```

Lisp implementations usually support compositions of up to four `car` and `cdr` operations. `carcer`, however, is braver.

```lisp
$ carcer "(cddadaddadadadaddddadaddddadaddr y)"
"(cddadaddadadadaddddadaddddadaddr y)" is equivalent to "(cdr (cdr (car (cdr (car (cdr (cdr (car (cdr (car (cdr (car (cdr (car (cdr (cdr (cdr (cdr (car (cdr (car (cdr (cdr (cdr (cdr (car (cdr (car (cdr (cdr y))))))))))))))))))))))))))))))"
```

## Why?

`carcer` is an example of a small parser written with the [nom](https://crates.io/crates/nom) parser combinator library.

## Error messages

`carcer` supplies reasonably good error messages:

```
$ carcer "(cdrar x"
0: at line 1:
(cdrar x
       ^
expected ')', found x

1: at line 1, in closing parenthesis:
(cdrar x
      ^
```