(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "13" inpt)
(get-test "13" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def points (-> inpt (string/split #"\n\r?\n\r?")
                     first
                     (string/split #",|\n\r?")
                     (->> (map string/trim)
                          (map to-int)
                          (partition 2))
                     set))

(def folds (-> inpt (string/split #"\n\r?\n\r?")
                    last
                    (string/split #" |=|\n\r?")
                    (->> (map string/trim)
                         (partition 4)
                         (map #(vector (nth % 2) (last %))))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn fold [coll i x?]
  (set
    (if x?
      (map #(let [x (first %) y (last %)]
              [(if (> x i) (- x (* 2 (- x i))) x) y]) coll)
      (map #(let [x (first %) y (last %)]
              [x (if (> y i) (- y (* 2 (- y i))) y)]) coll))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(println
  (count
    (fold points
          (to-int (last (first folds)))
          (= "x" (first (first folds))))))

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(defn task2 [points folds]
  (loop [paper points i 0]
    (if (>= i (count folds))
      paper
      (recur (fold paper
                   (to-int (last (nth folds i)))
                   (= "x" (first (nth folds i))))
             (inc i)))))

(let [res (task2 points folds)
      w (apply max (map first res))
      h (apply max (map last res))]
  (print2d
    (partition (inc w)
      (for [y (range (inc h)) x (range (inc w))]
        (if (contains? res [x y]) "#" ".")))))
