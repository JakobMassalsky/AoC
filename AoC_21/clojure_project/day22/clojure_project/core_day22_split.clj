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
            (->>
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

(defn c-split [[x1 x2 y1 y2 z1 z2] [x1' x2' y1' y2' z1' z2']]
  (remove nil?
    (let [xl (max x1 x1') xr (min x2 x2')
          yu (max y1 y1') yd (min y2 y2')
          zf (max z1 z1') zb (min z2 z2')]
      [(when (not= x1 xl) [x1 (dec xl) y1 y2 z1 z2])
       (when (not= x2 xr) [(inc xr) x2 y1 y2 z1 z2])
       (when (not= y1 yu) [xl xr y1 (dec yu) z1 z2])
       (when (not= y2 yd) [xl xr (inc yd) y2 z1 z2])
       (when (not= z1 zf) [xl xr yu yd z1 (dec zf)])
       (when (not= z2 zb) [xl xr yu yd (inc zb) z2])])))

(defn add-cube [cubes cube]
  (apply concat
    (for [c cubes :let [i (c-inter cube c)]]
        (if (nil? i) [c] (c-split c i)))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(loop [coll '() i 0]
  (if (>= i (count cubes))
    (println (reduce + (map c-vol coll)))
    (let [cube (rest (nth cubes i))
          add? (first (nth cubes i))
          c' (add-cube coll cube)]
      (recur (if (= add? "on") (conj c' cube) c')
             (inc i)))))
