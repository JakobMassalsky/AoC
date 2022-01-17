(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "11" inpt)
(get-test "11" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def dumbos (to-int-grid inpt))
(def h (count dumbos))
(def w (count (first dumbos)))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn newval
  [coll x y]
  (let [val (nth2d-bounds coll x y w h)]
    (if (not-in? (range 1 10) val) 0
      (+ val
         (reduce +
           (for [dy (range (dec y) (+ y 2))
                 dx (range (dec x) (+ x 2))]
              (if (> (nth2d-bounds coll dx dy w h) 9) 1 0)))))))


(defn do-flashes [coll]
  (loop [c coll]
    (let [flashed (partition w
                    (for [y (range h)
                          x (range w)]
                      (newval c x y)))]
      (if (= (count-n flashed 0) (count-n c 0))
        flashed
        (recur flashed)))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(loop [step 0 grid dumbos flashed 0]
  (let [newgrid (do-flashes (map #(map inc %) grid))]
    (when (= step 100) (println flashed)) ; Part01
    (if (= (count-n newgrid 1) (* w h))
        (println step) ; Part02
        (recur (inc step)
               newgrid
               (+ flashed (count-n newgrid 0))))))
