(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]))

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(get-input "04" inpt)

(def rolls
  (-> inpt string/split-lines
           (nth 0)
           (string/split #",")))

(def fieldsglob
              (-> inpt (string/split #"\r?\n\r?\n")
                       (->> (drop 1)
                            (map #(vec (string/split % #"\s+"))))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn updatefield
  [field roll]
  (map #(if (= % roll)
            "#"
            %)
    field))

(defn col
  [field i]
  (map #(nth field (+ i (* 5 %))) (range 5)))

(defn testfield
  [field]
  (loop [i 0 ret false]
    (if ret
      ret
      (when (< i 5)
        (recur (inc i) (or (every? #(= "#" %) (sublist field (* i 5) (+ 5 (* i 5))))
                           (every? #(= "#" %) (col field i))))))))

(defn calcfield
  [field]
  (reduce + (->> field (filter #(not= "" %))
                       (map #(string/replace % #"#" "0"))
                       (map read-string))))

(defn play-bingo
  [test]
  (loop [r 0 fields fieldsglob bingos ()]
    (if (or (>= r (count rolls))
            (test bingos))
      (do (println (nth rolls (- r 2)) (calcfield (last bingos)))
          (println (* (Integer/parseInt (nth rolls (- r 2))) (calcfield (last bingos)))))
      (recur (inc r)
             (map #(updatefield % (nth rolls r)) (filter #(not (testfield %)) fields))
             (concat bingos (filter testfield fields))))))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(play-bingo seq)

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(play-bingo #(= (count %) (count fieldsglob)))
