(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "19" inpt)
(get-test "19" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def probes (-> testinpt (string/split #"\r?\n\r?\n")
                (->> (map #(next (string/split % #",|\r?\n")))
                     (map #(partition 3 (map to-int %))))))
; (def probes (zipmap (range 100) probes))
(def bpp (count (first probes)))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(def overlap-n 12)
(defn orientations [[x y z]]
  [[x y z] [x (- y) (- z)] [(- x) (- y) z] [(- x) y (- z)]
   [(- x) z y] [x (- z) y] [x z (- y)] [(- x) (- z) (- y)]
   [y z x] [y (- z) (- x)] [(- y) (- z) x] [(- y) z (- x)]
   [(- y) x z] [y (- x) z] [y x (- z)] [(- y) (- x) (- z)]
   [z x y] [(- z) (- x) y] [z (- x) (- y)] [(- z) x (- y)]
   [(- z) y x] [z (- y) x] [z y (- x)] [(- z) (- y) (- x)]])

(defn get-all-rotations
  ([p s]
   (partition s
     (apply interleave
       (map orientations p))))
  ([p] (get-all-rotations p bpp)))

(defn transf-probe [p b]
  (map #(map - % b) p))

; (defn test-p2-beacons [p1 p2]
;   (let [matching (for [b2 p2
;                        :let [p2-transformed (set (transf-probe p2 b2))
;                              in (set/intersection (set p1) p2-transformed)]
;                        :when (>= (count in) overlap-n)]
;                    [1 (map - b2)])]
;     (if (empty? matching) nil (first matching))))
;
; (defn test-orientation [p1 p2]
;   (let [matching (for [b1 p1
;                        :let [p1-transformed (transf-probe p1 b1)
;                              [_ pos-p2] (test-p2-beacons p1-transformed p2)]
;                        :when (not (nil? pos-p2))]
;                    (map + b1 pos-p2))]
;     (if (empty? matching) nil (first matching))))


(defn test-orientation [p1 p2]
  (first? (for [b1 p1 b2 p2
                :let [p1-transformed (set (transf-probe p1 b1))
                      p2-transformed (set (transf-probe p2 b2))
                      in (set/intersection p1-transformed p2-transformed)]
                :when (>= (count in) overlap-n)]
            (map + b1 (map - b2)))))


(defn test-overlap [p1 p2]
  (first? (for [p2-rotated (get-all-rotations p2)
                :let [match (test-orientation p1 p2-rotated)]
                :when (not (nil? match))]
            [match p2-rotated])))

(defn find-match [p pcoll]
  (loop [i 0 matches []]
    (if (>= i (count pcoll)) matches
      (let [p2 (nth pcoll i)
            [rel-p2-pos p2o] (test-overlap p p2)]
        (recur (inc i)
          (if (nil? p2o) matches
            (conj matches [i rel-p2-pos p2o])))))))

(defn build-map [cur-probe search-space points cur-pos recur-lvl]
  (do (println "searching match for" cur-pos "in" (count search-space) "probes," recur-lvl "levels deep")
      (println "currently at" (count points) "beacons")
    (if (empty? search-space) points
      (let [matches (find-match cur-probe search-space)
            results (for [[i rel-p2-pos p2o] matches]
                      (build-map p2o
                        (concat (take i search-space) (drop (inc i) search-space))
                        (set/union points (set (transf-probe p2o (map - (map + cur-pos rel-p2-pos)))))
                        (map + cur-pos rel-p2-pos)
                        (inc recur-lvl)))]
        (println (count matches) "matches" recur-lvl "levels deep")
        (if (empty? matches) points (apply set/union results))))))


; (let [[i rel-p2-pos p2o] (find-match (first probes) (rest probes))]
;   (println (set/union (set (first probes)) (set (transf-probe p2o (map - rel-p2-pos))))))
; (build-map (first probes) (rest probes) (set (first probes)) [0 0 0] 0)
; (println (build-map (nth probes 0) [(nth probes 1)(nth probes 4)] (set (first probes)) [0 0 0] 0));

; (println (nth probes 2))
; (find-match (nth probes 2) (concat (take 2 probes) (drop 3 probes)))
(test-overlap (nth probes 4) (nth probes 1))
; (test-overlap (nth probes 2) (nth probes 1))
; (get-all-rotations [[1 2 3] [4 5 6]] 2)
; (defn rot [[x y z]] [(- z) x (- y)])
; (println (map rot (nth probes 4)))
; (test-orientation (nth probes 1) (map rot (nth probes 4)))
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
