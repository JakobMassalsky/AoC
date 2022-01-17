(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "14" inpt)
(get-test "14" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def start (first (string/split-lines inpt)))

(def rules (apply hash-map
             (map string/trim
               (drop 2
                 (string/split inpt #" -> |\n\r?")))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn get-rule-res [rule]
  (map string/join (partition 2 1 [(first rule) (rules rule) (last rule)])))

(def rule-res (zipmap (keys rules) (map get-rule-res (keys rules))))

(defn update-duo-cnts [duo-cnts]
  (apply hash-map
    (flatten
      (for [k (keys duo-cnts)]
        [k (apply +
             (map duo-cnts
               (filter #(in? (rule-res %) k)
                 (keys rule-res))))]))))

(defn to-duos [poly]
  (concat (map string/join (partition 2 1 poly))))

(defn count-char [ch duos]
  (apply + (map duos (filter #(= (str (first %)) ch) (keys duos)))))
(defn char-counts [duo-cnts]
  (map #(+ (count-char % duo-cnts) (if (= % (str (last start))) 1 0)) (vals rules)))

(def dc (zipmap (keys rules)
          (map #(count (filter #{%} (to-duos start))) (keys rules))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn do-steps [steps duo-cnts]
  (loop [dc duo-cnts i 0]
    (if (>= i steps)
      (let [cc (char-counts dc)]
        (- (apply max cc)
           (apply min cc)))
      (recur (update-duo-cnts dc)
             (inc i)))))

(println (do-steps 10 dc))
(println (do-steps 40 dc))
