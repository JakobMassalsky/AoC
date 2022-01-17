(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]))
(get-input "02" inpt)

(def values
  (map #(Integer/parseInt (str %))
    (map last
      (clojure.string/split-lines inpt))))

(def dirs
  (map first
    (clojure.string/split-lines inpt)))

;; Part 1

(defs x 0 y 0)

(dotimes [i (count values)]
  (let [d (nth dirs i), v (nth values i)]
    (when (= \d d) (+= y v))
    (when (= \u d) (-= y v))
    (when (= \f d) (+= x v))))

(println [x y (* x y)])

;; Part 2

(defs x 0 y 0 aim 0)

(dotimes [i (count dirs)]
  (let [d (nth dirs i), v (nth values i)]
    (when (= \d d) (+= aim v))
    (when (= \u d) (-= aim v))
    (when (= \f d) (+= x v)
                   (+= y (* aim v)))))

(println [x y (* x y)])
