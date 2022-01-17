(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "20" inpt)
(get-test "20" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def enha (-> inpt (string/split #"\r?\n\r?\n") first
            (string/replace #".|#" {"." "0" "#" "1"})))

(def img (-> inpt (string/split #"\r?\n\r?\n") last
            (string/replace #".|#" {"." "0" "#" "1"})
            (->> clojure.string/split-lines
                 (map #(clojure.string/split % #"")))))

(def outers {1 \1 0 \0})

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn inc-size [img filler]
  (let [w (count (first img))
        zeros (vec (repeat w filler))]
    (mapv #(vec (conj (vec (cons filler %)) filler))
      (vec (conj (vec (cons zeros img)) zeros)))))

(defn pixel-string [img x y w h filler]
  (apply str
    (for [y' (range (dec y) (+ y 2))
          x' (range (dec x) (+ x 2))]
      (nth2d-bounds img x' y' w h filler))))

(defn new-val [s]
  (nth enha (read-string (str "2r" s))))

(defn update-img [img filler]
  (let [img' (inc-size img filler)
        w (count (first img'))
        h (count img')]
    (mapv vec
      (partition w
        (for [y (range h)
              x (range w)]
          (new-val (pixel-string img' x y w h filler)))))))

(defn count-image [img]
  (count (filter #{"1" \1} (flatten img))))

(defn image-seq [img i]
  (loop [j 0 im img]
    (if (>= j i) im
      (recur (inc j)
             (update-img im (outers (mod j 2)))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(println (count-image (image-seq img 2)))

(println (count-image (image-seq img 50)))
