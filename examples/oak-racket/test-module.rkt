#lang racket

;; Test module system
(require racket/base)
(require "other-module.rkt")

(provide my-function)
(provide (all-defined-out))

(define (my-function x)  
  (+ x 1))

(define (another-function y)
  (* y 2))
