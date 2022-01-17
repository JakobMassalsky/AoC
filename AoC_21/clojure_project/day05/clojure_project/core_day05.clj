(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "05" inpt)
(get-test "05" inpttest)


;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(defn horr-or-vert? [points]
  (or (= (nth points 0) (nth points 2))
      (= (nth points 1) (nth points 3))))

(defn to-int-line [points]
  (map #(Integer/parseInt %) points))

(def all-lines
  (-> inpt string/split-lines
           (->>
             (map #(string/split % #"(,| -> )"))
             (map to-int-line))))

(def lines-no-diag (filter horr-or-vert? all-lines))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn line-to-pointset [line]
  (let [x1 (nth line 0) x2 (nth line 2)
        y1 (nth line 1) y2 (nth line 3)]
    (set (map vector (to-range-inc x1 x2) (to-range-inc y1 y2)))))

(defn print-intersects [lines]
  (loop [i 0 oneset #{} moreset #{}]
    (if (>= i (count lines))
      (println (count moreset))
      (let [newpoints (line-to-pointset (nth lines i))]
        (recur (inc i)
               (set/union oneset newpoints)
               (set/union moreset (set/intersection oneset newpoints)))))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(print-intersects lines-no-diag)

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(print-intersects all-lines)
