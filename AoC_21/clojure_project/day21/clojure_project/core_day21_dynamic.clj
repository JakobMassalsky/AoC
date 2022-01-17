(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))


;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def inpt [0 2])
(def testinpt [3 7])

(def part2-win 28)

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def dice (cycle (for [i (range 1 301 3)
                       :let [thr (map #(inc (mod (dec %) 100))
                                   (range i (+ i 3)))]]
                   (apply + thr))))

(def ddice {3 1, 4 3, 5 6, 6 7, 7 6, 8 3, 9 1})

(def p1state {[(first inpt) 0] 1})
(def p2state {[(second inpt) 0] 1})

(defn new-state [state]
  (merge-with +
    (for [[[pos score] v] state]
      (into {}
        (for [[roll amount] ddice
              :let [npos (mod (+ pos roll) 10)]]
          [[npos (+ score (inc npos))] (* v amount)])))))

(defn state-seq [state]
  (take-while #(> (count (keys %))) 0) (iterate new-state p1state))

(print2d (state-seq p1state))

; (defn wins-not-wins [state-seq n]
;   (for [i (range (inc n))]
;     []))
; (defn take-turns [state n]
;   ())

(defn count-wins [state p1?]
    (apply +
      (for [[[_ p1s _ p2s] v] state
            :when (if p1? (>= p1s part2-win)
                          (>= p2s part2-win))]
         v)))

(defn clean-wins [state]
  (into {} (for [k (keys state)
                 :let [[_ p1s _ p2s] k]
                 :when (and (< p1s part2-win)
                            (< p2s part2-win))]
             [k (state k)])))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(loop [p1s 0 p2s 0 pos1 (first inpt) pos2 (second inpt) turn 0 p1? true]
  (if (or (> p1s 999) (> p2s 999))
    [p1s p2s (* 3 turn)]
    (let [d (nth dice turn)
          pos1' (if p1? (mod (+ pos1 d) 10) pos1)
          pos2' (if p1? pos2 (mod (+ pos2 d) 10))]
      (recur (if p1? (+ p1s (inc pos1')) p1s)
             (if p1? p2s (+ p2s (inc pos2')))
             pos1' pos2' (inc turn) (not p1?)))))

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

; (time (loop [state gamestate turn 0 p1? true wins [0 0]]
;         (if (= (count state) 0) wins
;           (let [state' (update-game state p1?)
;                 p1wins (count-wins state' true)
;                 p2wins (count-wins state' false)
;                 state'' (clean-wins state')]
;             (recur state'' (inc turn) (not p1?)
;                    (map + wins [p1wins p2wins]))))))
