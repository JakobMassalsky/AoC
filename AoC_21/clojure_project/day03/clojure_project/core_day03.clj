(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]))

(get-input "03" inpt)

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(def numVec (-> inpt
                (clojure.string/replace #"\n+" "")
                (clojure.string/split "")))

; (println (first numVec))


;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;
