(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]))

(get-input "06" inpt)
(get-test "06" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def agelist
  (-> inpt string/split-lines
           (nth 0)
           (string/split #",")
           (->>
             (map #(Integer/parseInt %)))))

(def ages (mapv #(count (filter #{%} agelist)) (range 9)))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn calcpop [fish numdays]
  (loop [i 0 age fish]
    (if (>= i numdays)
      (println (reduce + age))
      (recur (inc i)
             (assoc
               (vec (concat (drop 1 age) (take 1 age))) 6
               (+ (first age) (nth age 7)))))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(calcpop ages 80)

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(calcpop ages 256)
