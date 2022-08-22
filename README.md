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
$ carcer "()"
0: at line 1:
()
 ^
expected 'c', found )

1: at line 1, in car or cdr:
()
 ^

$ carcer "(ca)"
0: at line 1:
(ca)
   ^
expected 'r', found )

1: at line 1, in car or cdr:
(ca)
   ^

$ carcer "(cdar x"
0: at line 1:
(cdar x
       ^
expected ')', found x

1: at line 1, in closing parenthesis:
(cdar x
      ^
```

## Building

This project requires Rust 1.58. You can get it at [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/vrmiguel/carcer
cargo install --path carcer
```