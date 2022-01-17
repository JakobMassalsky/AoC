(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "09" inpt)
(get-test "09" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def lines (to-int-grid inpt))
(def h (count lines))
(def w (count (first lines)))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn nth2dh
  [coll x y]
  (if (and (and (>= x 0) (< x w))
           (and (>= y 0) (< y h)))
    (nth (nth coll y) x)
    9999))

(defn lowest?
  [coll x y]
  (let [val (nth2dh coll x y)]
    (and (> (min (nth2dh coll (inc x) y)
                 (nth2dh coll (dec x) y)
                 (nth2dh coll x (inc y))
                 (nth2dh coll x (dec y))) val))))

(defn basin-size [coll x y visited]
  (if (and (< x w) (< y h) (>= x 0) (>= y 0)
           (not (contains? visited [x y]))
           (< (nth2dh coll x y) 9))
    (basin-size coll x (dec y)
      (basin-size coll x (inc y)
        (basin-size coll (dec x) y
          (basin-size coll (inc x) y
            (conj visited [x y])))))
    visited))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(loop [x 0 y 0 lowpoints () basins #{}]
  (if (> y (dec h))
    (do (println (reduce + (map inc lowpoints))) ; Part01
      (println (reduce * (take 3 (reverse (sort (map count basins))))))) ; Part02
    (let [r (lowest? lines x y)]
       (recur (if (= x (dec w)) 0 (inc x))
              (if (= x (dec w)) (inc y) y)
              (if r (conj lowpoints (nth2dh lines x y))
                lowpoints)
              (if r (conj basins (basin-size lines x y #{}))
                basins)))))
