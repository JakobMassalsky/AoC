(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "12" inpt)
(get-test "12" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def paths (partition 2
             (map string/trim
               (concat (string/split inpt #"-|\n\r?")
                       (reverse (string/split inpt #"-|\n\r?"))))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def dests
  (into (hash-map)
    (for [k (set (map first paths))]
      [k (set (map last (filter #(= (first %) k) paths)))])))

(def lower? #(= % (string/lower-case %)))

(defn find-end [start visited part2?]
  (if (= start "end") 1
    (let [v (conj visited (when (lower? start) start))]
      (reduce +
        (for [k (dests start)]
          (if-not (contains? visited k)
            (find-end k v part2?)
            (if-not (and part2? (not= k "start")) 0
              (if (not= k "start") (find-end k (disj v k) false)))))))))

(println (find-end "start" #{} false)) ; Part01
(println (find-end "start" #{} true)) ; Part02
