(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "25" inpt)
(get-test "25" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def li (string/split-lines inpt))
(def w (count (first li)))
(def h (count li))

(def cc
  (into {} (for [y (range h) x (range w)
                 :let [e (nth (nth li y) x)]
                 :when (not= \. e)]
             [[x y] e])))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn add-mod [[x1 y1] [x2 y2]]
  [(mod (+ x1 x2) w) (mod (+ y1 y2) h)])

(defn advance [m c dir]
  (into {}
    (for [[k v] m
          :let [nk (add-mod k dir)]]
      (if (and (= c v) (nil? (m nk)))
        [nk v] [k v]))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(loop [m cc i 1]
  (let [m' (advance m \> [1 0])
        m'' (advance m' \v [0 1])]
    (if (= m'' m) [i]
      (recur m'' (inc i)))))
