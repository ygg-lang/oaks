#lang racket

;; Basic syntax
(define (factorial n)
    (if (zero? n)
        1
        (* n (factorial (- n 1)))))

(define (fib n)
    (cond
        [(<= n 1) n]
        [else (+ (fib (- n 1)) (fib (- n 2)))]))

;; Lists and quoting
(define lst '(1 2 3 4 5))
(define quoted-symbol 'symbol)
(define quasiquoted `(1 2 ,(+ 1 2)))

;; Let bindings
(let ([x 10]
      [y 20])
    (+ x y))

(let* ([x 10]
       [y (+ x 10)])
    y)

(letrec ([is-even? (lambda (n)
             (or (zero? n)
                 (is-odd? (- n 1))))]
         [is-odd? (lambda (n)
             (and (not (zero? n))
                 (is-even? (- n 1))))])
    (is-even? 4))

;; Structs
(struct point (x y))
(define p (point 10 20))
(point-x p)

;; Modules
(module my-module racket
    (provide my-func)
    (define (my-func x) x))

;; Macros
(define-syntax-rule (swap x y)
    (let ([tmp x])
        (set! x y)
        (set! y tmp)))

;; Strings and Characters
(define s "Hello, Racket!")
(define c #\A)
(define newline #\newline)

;; Booleans
(define t #t)
(define f #f)

;; Numbers
(define n1 123)
(define n2 123.456)
(define n3 1/2)
(define n4 1+2i)
