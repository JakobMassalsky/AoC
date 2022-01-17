(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "18" inpt)
(get-test "18" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def nums [\0 \1 \2 \3 \4 \5 \6 \7 \8 \9])

(defn parse-sf-num [numstring]
  (loop [i 0 level 0 numlist []]
    (if (>= i (count numstring))
      numlist
      (let [n (filter #{(nth numstring i)} nums)
            level' (case (nth numstring i)
                     \[ (inc level)
                     \] (dec level)
                     level)]
        (recur (inc i) level'
          (if (empty? n) numlist
            (conj numlist [(to-int (str (first n))) level'])))))))

(def numbers (-> inpt string/split-lines))
(def sf-numbers (map parse-sf-num numbers))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn explodesn [sn i]
  (let [left (take i sn) [n1 l] (nth sn i)
        [n2 _] (nth sn (inc i)) right (drop (+ i 2) sn)]
    (vec (concat (drop-last left)
                 (when-not (empty? left)
                   [[(+ (first (last left)) n1) (last (last left))]])
                 [[0 (dec l)]]
                 (when-not (empty? right)
                   [[(+ (first (first right)) n2) (last (first right))]])
                 (rest right)))))

(defn splitsn [sn i]
  (let [[n l] (nth sn i) q (quot n 2)]
    (vec (concat (take i sn)
           [[q (inc l)] [(+ q (mod n 2)) (inc l)]]
           (drop (inc i) sn)))))

(defn reduce-sn [sn]
  (let [b (empty? (filter #(> (last %) 4) sn))]
    (loop [i 0]
      (if (>= i (count sn)) sn
        (let [[n l] (nth sn i)]
          (if (> l 4) (explodesn sn i)
            (if (and b (> n 9)) (splitsn sn i)
              (recur (inc i)))))))))

(def inc-lvl #(vector (first %) (inc (last %))))

(defn addsn [snc]
  (if (= (count snc) 1) (first snc)
    (apply-till-equal reduce-sn
      (mapv inc-lvl (concat (addsn (drop-last snc)) (last snc))))))

(defn mag-sn [sn]
  (loop [i 1]
    (if (>= i (count sn)) sn
      (let [[n1 l1] (nth sn (dec i))
            [n2 l2] (nth sn i)
            n (+ (* 3 n1) (* 2 n2))]
        (if (= l1 l2)
          (vec (concat (take (dec i) sn) [[n (dec l1)]] (drop (inc i) sn)))
          (recur (inc i)))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(println (apply-till-equal mag-sn (addsn sf-numbers))) ; part 1
(apply max
  (map #(->> % addsn (apply-till-equal mag-sn) first first)
       (cart sf-numbers sf-numbers))) ; part 2
