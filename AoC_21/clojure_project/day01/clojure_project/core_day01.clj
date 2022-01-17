(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]))
(get-input "01" inpt)

(def parsedinpt (map #(Integer/parseInt %) (clojure.string/split-lines inpt)))

; Part 01

(def acc 0)
(dotimes [i (- (count parsedinpt) 1)]
  (when (< (nth parsedinpt i) (nth parsedinpt (inc i)))
     (++ acc)))

(println acc)

; Part 02

(def acc 0)
(dotimes [i (- (count parsedinpt) 3)]
  (when (< (nth parsedinpt i) (nth parsedinpt (+ i 3)))
       (++ acc)))

(println acc)
