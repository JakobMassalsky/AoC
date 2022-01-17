(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "24" inpt)
(get-test "24" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def parse-numbers #(if (nil? (re-matches #"-?\d+" %)) % (to-int %)))

(def prog (-> inpt string/split-lines
            (->> (map #(string/split % #"\s"))
                 (map #(map parse-numbers %)))))

; (println prog)

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn inp [[a v] mem]
  (assoc mem a v))

(defn add [[a b] mem]
  (update mem a + (if (int? b) b (mem b))))

(defn mul [[a b] mem]
  (update mem a * (if (int? b) b (mem b))))

(defn div [[a b] mem]
  (update mem a quot (if (int? b) b (mem b))))

(defn md [[a b] mem]
  (update mem a mod (if (int? b) b (mem b))))

(defn equ [[a b] mem]
  (assoc mem a (if (= (mem a) (if (int? b) b (mem b))) 1 0)))

(def ops {"add" add "mul" mul "div" div "mod" md "eql" equ "inp" inp})

(defn exec [prog inptq]
  (loop [i 0 mem {"w" 0 "x" 0 "y" 0 "z" 0} q inptq]
    (if (= i (count prog)) mem
      (let [l (nth prog i)
            instr (first l)
            args (if (= instr "inp")
                   [(last l) (first q)]
                   (rest l))]
        ; (println instr args mem)
        (recur (inc i)
          ((ops instr) args mem)
          (if (= instr "inp") (rest q) q))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(def r10 (map inc (reverse (range 9))))

(def r (flatten (for [a r10 b r10 c r10 d r10 e r10 f r10 g r10 h r10 i r10 j r10 k r10 l r10 m r10 n r10]
                  ; ((exec prog [a b c d e f g h i j k l m n]) "z"))))
                  ((exec prog (repeat 9)) "z"))))

(println (first r))
