(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "08" inpt)
(get-test "08" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def mappings
  (-> inpt string/split-lines
      (->> (map #(nth (string/split % #" \| ") 0))
           (map #(string/split % #" "))
           (map #(map set %)))))

(def nums
  (-> inpt string/split-lines
      (->> (map #(nth (string/split % #" \| ") 1))
           (map #(string/split % #" "))
           (map #(map set %)))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn combine-number [numseq]
  (reduce +
    (map #(* (nth numseq %) (Math/pow 10 (- (count numseq) (inc %))))
         (range (count numseq)))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(println (map #(count (filter #{%} (map count (flatten nums)))) (range 8)))

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(loop [i 0 acc 0]
  (let [mapping (nth mappings i)
        k1 (first (filter #(= (count %) 2) mapping))
        k4 (first (filter #(= (count %) 4) mapping))
        k7 (first (filter #(= (count %) 3) mapping))
        k8 (first (filter #(= (count %) 7) mapping))
        k9 (first (filter #(and (set/subset? k4 %) (= (count %) 6)) mapping))
        k3 (first (filter #(and (set/subset? k1 %) (= (count %) 5)) mapping))
        k0 (first (filter #(and (= (count (set/intersection % k3)) 4) (set/subset? k7 %)) mapping))
        k6 (first (filter #(and (and (not= % k0) (not= % k9)) (= (count %) 6)) mapping))
        k2 (first (filter #(and (= (count (set/intersection % k4)) 2) (= (count %) 5)) mapping))
        k5 (first (filter #(and (and (not= % k2) (not= % k3)) (= (count %) 5)) mapping))
        number (map #(condp = % k0 0 k1 1 k2 2 k3 3 k4 4 k5 5 k6 6 k7 7 k8 8 k9 9) (nth nums i))
        res (+ acc (combine-number number))]
    (println i res)
    (when (< i (count mappings))
      (recur (inc i) res))))
