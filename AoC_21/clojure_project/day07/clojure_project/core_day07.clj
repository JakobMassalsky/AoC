(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]))

(get-input "07" inpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def hpos
  (-> inpt string/split-lines
           (nth 0)
           (string/split #",")
           (->>
             (map #(Integer/parseInt %)))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def func1 #(do %))
(def func2 #(/ (* % (inc %)) 2))

(defn reducesums [coll func]
  (reduce + (map func (map #(Math/abs %) coll))))

(defn getdist [coll func]
  (loop [nums (map dec coll) lastoff (reducesums coll func)]
    (let [newoff (reducesums nums func)]
      (if (> newoff lastoff)
        (do (println lastoff))
        (recur (map dec nums) newoff)))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(getdist hpos func1)

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(getdist hpos func2)
