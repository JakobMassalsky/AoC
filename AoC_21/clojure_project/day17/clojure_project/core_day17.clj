(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(defs xmin 20 xmax 30 ymin -10 ymax -5) ;testinput
(defs xmin 135 xmax 155 ymin -102 ymax -78) ;myinput

(defn xyfilter [[xvel yvel]]
  (loop [x 0 xv xvel y 0 yv yvel]
    (if (or (< y ymin) (> x xmax) (and (< x xmin) (= xv 0))) false
      (if (and (<= y ymax) (>= y ymin) (<= x xmax) (>= x xmin)) true
        (recur (+ x xv) (max 0 (dec xv)) (+ y yv) (dec yv))))))

(println (sum-to-n (dec (- ymin)))) ; Part01
(count (filter xyfilter (cart (range 0 (inc xmax))
                              (range ymin (- ymin))))) ; Part02
