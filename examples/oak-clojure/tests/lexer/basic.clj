;; Comprehensive Clojure Lexer Test
(ns com.example.core
    "A comprehensive test namespace."
    (:require [clojure.string :as str]
              [clojure.set :refer [union intersection]])
    (:import [java.util Date Calendar]))

;; Constants and Defs
(def ^:const pi 3.14159)
(def global-config {:env "prod" :retries 3})

;; Function Definition with Docstring and Multi-arity
(defn greet
    "Greets the user with a message.
     Supports optional title."
    ([name] (str "Hello, " name "!"))
    ([name title] (str "Hello, " title " " name "!")))

;; Records and Protocols
(defrecord Person [name age])

(defprotocol Speaker
    (speak [this] "Make the object speak"))

(extend-type Person
    Speaker
    (speak [this] (str (:name this) " says hello.")))

;; Main Function
(defn -main [& args]
    (let [users [{:name "Alice" :age 30}
                 {:name "Bob" :age 25}
                 {:name "Charlie" :age 35}]
          threshold 26
          filtered (filter #(> (:age %) threshold) users)
          sorted (sort-by :age users)]
        
        (println "Filtered Users:" filtered)
        
        ;; Conditionals
        (if (> (count users) 0)
            (println "Users found")
            (println "No users"))
            
        (cond
            (< threshold 20) "Low"
            (< threshold 30) "Medium"
            :else "High")
        
        ;; Macros
        (when-let [x (first users)]
            (println "First user:" x))
            
        ;; Collections and Literals
        (def my-map {:a 1, :b 2, "c" 3})
        (def my-vec [1 2 3 :keyword])
        (def my-set #{1 2 3})
        (def my-list '(1 2 3))
        
        ;; Threading macros
        (->> users
             (map :age)
             (filter even?)
             (reduce +))
             
        ;; Metadata
        (meta #'greet)
        
        ;; Java Interop
        (.toUpperCase "clojure")
        (Date.)
        
        ;; Regex
        (re-matches #"\d+" "123")
        
        ;; Anonymous function syntax
        #(+ % 1)
        
        ;; Keywords and Symbols
        :keyword
        ::namespaced-keyword
        'symbol
        
        ;; Numbers
        123
        -123
        123N
        1.23
        1.23M
        1.23e-4
        1/3
        0xFF
        012
        2r1010
        
        ;; Characters
        \a \newline \space \u0041
    ))

;; Macro Definition
(defmacro unless [test & body]
    `(if (not ~test)
         (do ~@body)))

;; Comment
(comment
    (greet "World")
    (speak (->Person "Dave" 40)))
