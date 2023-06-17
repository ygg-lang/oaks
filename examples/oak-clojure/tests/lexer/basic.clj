;; Clojure test file for lexer testing

;; Basic function definition
(defn greet [name]
  (str "Hello, " name "!"))

;; Variable binding
(def pi 3.14159)
(def users ["Alice" "Bob" "Charlie"])

;; Anonymous function
(#(+ %1 %2) 10 20)

;; Map and filter operations
(map inc [1 2 3 4 5])
(filter even? [1 2 3 4 5 6])

;; Let binding
(let [x 10
      y 20
      sum (+ x y)]
  (println "Sum is:" sum))

;; Cond statement
(defn classify-number [n]
  (cond
    (< n 0) "negative"
    (= n 0) "zero"
    (< n 10) "small positive"
    :else "large positive"))

;; Loop and recur
(defn factorial [n]
  (loop [cnt n acc 1]
    (if (zero? cnt)
      acc
      (recur (dec cnt) (* acc cnt)))))

;; Namespace declaration
(ns myapp.core
  (:require [clojure.string :as str])
  (:use [clojure.set]))

;; Protocol definition
(defprotocol Shape
  (area [this])
  (perimeter [this]))

;; Record definition
(defrecord Rectangle [width height]
  Shape
  (area [this] (* width height))
  (perimeter [this] (* 2 (+ width height))))

;; Macro definition
(defmacro unless [test body]
  `(if (not ~test) ~body))

;; Comment
;; This is a single line comment

#_(This is a comment using the discard reader macro)