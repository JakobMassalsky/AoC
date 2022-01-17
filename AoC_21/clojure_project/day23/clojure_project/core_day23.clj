(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]
            [clojure.data.priority-map :refer [priority-map]]))

; (get-input "XX" inpt)
; (get-test "XX" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def ti1 [[0 0 0 0 0 0 0] [[2 1] [3 4] [2 3] [4 1]]])
(def ti2 [[0 0 0 0 0 0 0] [[2 4 4 1] [3 3 2 4] [2 2 1 3] [4 1 3 1]]])
(def mi1 [[0 0 0 0 0 0 0] [[2 2] [3 3] [1 4] [4 1]]])
(def mi2 [[0 0 0 0 0 0 0] [[2 4 4 2] [3 3 2 3] [1 2 1 4] [4 1 3 1]]])
(def s 2)
;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def abs-inc-neg #(if (< % 1) (inc (- %)) %))
(def energy #(* (Math/pow 10 (dec %1)) %2))

(defn format-sublist [hw i a]
  (if (= i a) '()
    (if (<= a i) (sublist hw (inc a) i)
      (sublist hw (inc i) (inc a)))))

(defn skips [start end]
  (if (or (= end 0) (= end 6)
          (= start 0) (= start 6))
    -1 0))

(defn hallway-pos
  [hw i amph]
  (let [pos (inc i)
        free-right (count (take-while #{0} (drop (inc pos) hw)))
        free-left (count (take-while #{0} (reverse (take (inc pos) hw))))]
    (for [d (range (- free-left) free-right)
          :let [vi (inc (+ d pos))
                ab (abs-inc-neg (inc d))]]
      [(assoc hw vi amph) (+ (skips pos vi) (* 2 ab))])))

(defn move-to-home [hw am]
  (first
    (for [i (range (count hw))
          :let [a (hw i)]
          :when (> a 0)
          :let [home (am (dec a))]
          :when (and (= (filter #{a} home) home)
                     (= 0 (apply + (format-sublist hw i a)))) ;;;;;
          :let [d (abs-inc-neg (- i a))]]
      [[(assoc hw i 0) (assoc am (dec a) (vec (cons a home)))]
       (energy a (+ (- (dec s) (count home)) (skips i a) (* d 2)))])))

(defn check-for-home [hw i amph home]
  (and (not= i (dec amph))
       (= (filter #{amph} home) home)
       (= 0 (apply + (sublist hw (+ 2 i) (inc amph))))))

(defn move-amph [hw am deni homei]
  (let [den (am deni) home (am homei)
        amph (first den)
        at-home (assoc am homei (vec (cons amph home)))]
    [[hw (assoc at-home deni (vec (rest den)))]
     (energy amph
       (+ (- s (count den)) (- s (count home)) (inc (* 2 (Math/abs (- deni homei))))))]))

(defn any-home [hw am]
  (first
    (for [i (range 4)
          :let [amph (first (am i))]
          :when (and (not (nil? amph))
                  (check-for-home hw i amph (am (dec amph))))]
      (move-amph hw am i (dec amph)))))

(defn neighbours [[hw am]]
  (apply hash-map
    (if-let [n (any-home hw am)] n
      (if-let [n (move-to-home hw am)] n
        (apply concat
          (for [i (range 4)
                :let [amph (first (am i))]
                :when (not (nil? amph))
                :let [st (- s (count (am i)))
                      positions (hallway-pos hw i amph)]
                :when (or (not= i (dec amph))
                          (and (= i (dec amph))
                            (not= (filter #{amph} (am i)) (am i))))]
            (mapcat vector (map vector (map first positions)
                             (repeat (assoc am i (vec (rest (am i))))))
              (->> positions (map last) (map #(energy amph (+ st %)))))))))))

(defn remove-keys [m pred]
  (select-keys m (filter (complement pred) (keys m))))

(defn dijkstra
  [start f]
  (loop [q (priority-map start 0) r {}]
    (if-let [[v d] (peek q)]
      (let [dist (-> (f v) (remove-keys r) (fmap (partial + d)))]
        (recur (merge-with min (pop q) dist) (assoc r v d)))
      r)))


;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(println ((dijkstra mi1 neighbours) [[0 0 0 0 0 0 0] [[1 1] [2 2] [3 3] [4 4]]]))
(def s 4)
(println ((dijkstra mi2 neighbours) [[0 0 0 0 0 0 0] [[1 1 1 1] [2 2 2 2] [3 3 3 3] [4 4 4 4]]]))
