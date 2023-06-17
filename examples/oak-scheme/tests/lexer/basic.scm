;; Scheme test file
(define (factorial n)
  (if (<= n 1)
      1
      (* n (factorial (- n 1)))))

(define (fibonacci n)
  (cond
    ((= n 0) 0)
    ((= n 1) 1)
    (else (+ (fibonacci (- n 1))
             (fibonacci (- n 2))))))

;; Lists
(define numbers '(1 2 3 4 5))
(define squares (map (lambda (x) (* x x)) numbers))

;; Higher-order functions
(define (apply-twice f x)
  (f (f x)))

(define (add-one x) (+ x 1))

;; Test expressions
(display "Factorial of 5: ")
(display (factorial 5))
(newline)

(display "Fibonacci of 10: ")
(display (fibonacci 10))
(newline)

(display "Squares: ")
(display squares)
(newline)

(display "Apply twice add-one to 3: ")
(display (apply-twice add-one 3))
(newline)