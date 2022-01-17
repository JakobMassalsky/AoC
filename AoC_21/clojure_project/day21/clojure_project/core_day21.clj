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

(def gamestate {[(first inpt) 0 (second inpt) 0] 1})

(defn new-keys [state p1?]
  (set (for [k (keys state)
             roll (keys ddice)
             :let [[p1p p1s p2p p2s] k
                   npos (mod (+ (if p1? p1p p2p) roll) 10)]]
         (if p1? [npos (+ p1s (inc npos)) p2p p2s]
                 [p1p p1s npos (+ p2s (inc npos))]))))

(defn update-game [state p1?]
  (let [nkeys (new-keys state p1?)]
    (into {}
      (for [k nkeys
            :let [[p1p p1s p2p p2s] k]]
        [k (apply +
             (for [roll (keys ddice)]
               (* (ddice roll)
                  (if p1?
                    (if-let [v (state [(mod (- p1p roll) 10)
                                       (- p1s (inc p1p))
                                       p2p p2s])] v 0)
                    (if-let [v (state [p1p p1s
                                       (mod (- p2p roll) 10)
                                       (- p2s (inc p2p))])] v 0)))))]))))

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

(time (loop [state gamestate turn 0 p1? true wins [0 0]]
        (if (= (count state) 0) wins
          (let [state' (update-game state p1?)
                p1wins (count-wins state' true)
                p2wins (count-wins state' false)
                state'' (clean-wins state')]
            (recur state'' (inc turn) (not p1?)
                   (map + wins [p1wins p2wins]))))))
