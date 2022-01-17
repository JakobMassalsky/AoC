(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "22" inpt)
(get-test "22" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def cubes (-> inpt string/split-lines
            (->> ;(take 10)
              (map #(string/split % #" ?,?.=|\.\."))
              flatten
              (partition 7))))

(def cubes (map cons (map first cubes)
             (map #(map to-int %) (map rest cubes))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def between #(and (>= %1 %2) (<= %1 %3)))

(defn c-vol [cube]
  (apply * (map #(- (inc (last %)) (first %))
             (partition 2 cube))))

(defn c-inter [[x1 x2 y1 y2 z1 z2] [x1' x2' y1' y2' z1' z2']]
    (when (and (or (between x1' x1 x2) (between x1 x1' x2'))
               (or (between y1' y1 y2) (between y1 y1' y2'))
               (or (between z1' z1 z2) (between z1 z1' z2')))
      [(max x1' x1) (min x2' x2)
       (max y1' y1) (min y2' y2)
       (max z1' z1) (min z2' z2)]))

(defn all-inters [coll c]
  (for [i (map #(c-inter c %) coll)
        :when (not (nil? i))] i))

(defn get-inter-vol [cubes]
  (if (empty? cubes) 0
    (loop [c1 (first cubes) crest (rest cubes) s 0]
      (if (empty? crest) (+ s (- (c-vol c1)))
        (let [its (all-inters crest c1)
              s' (+ s (- (c-vol c1)) (- (get-inter-vol its)))]
          (recur (first crest) (rest crest) s'))))))

(defn add-cube [cubes cube on?]
  (if (not on?) 0
    (let [vi (get-inter-vol (all-inters cubes cube))]
      (+ (c-vol cube) vi))))

(defn cube-vol [cubes]
  (apply +
    (for [i (range (count cubes))
          :let [c (nth cubes i)]]
      (add-cube (map rest (take i cubes))
        (rest c) (= (first c) "on")))))


;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(println (cube-vol (reverse (take 20 cubes)))) ;part 1
(println (time (cube-vol (reverse cubes)))) ;part 2
