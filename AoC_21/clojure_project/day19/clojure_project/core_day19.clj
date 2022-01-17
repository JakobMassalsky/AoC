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
(println bpp)

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

(defn test-p2-beacons [p1 p2]
  (let [sp1 (set p1)]
    (loop [i 0]
      (if (>= i (count p2)) nil
        (let [b2 (nth p2 i)
              p2tr (transf-probe p2 b2)
              inters (set/intersection sp1 (set p2tr))]
          (if (>= (count inters) overlap-n)
            [p2tr (map - b2)]
            (recur (inc i))))))))

(defn test-orientation [p1 p2]
  (loop [i 0]
    (if (>= i (count p1)) nil
      (let [test-beac (nth p1 i)
            [_ p2-pos] (test-p2-beacons (transf-probe p1 test-beac) p2)]
        (if-not (nil? p2-pos) ;
          ; [(transf-probe p2-rel-tb (map - tb))
          (map + test-beac p2-pos) ; p2-rel-tb (map + tb p2-pos)]
          (recur (inc i)))))))

(defn test-overlap [p1 p2]
  (let [p2rotations (get-all-rotations p2)]
    (loop [i 0]
      (if (>= i (count p2rotations)) nil
        (let [p2o (nth p2rotations i)
              rel-p2-pos (test-orientation p1 p2o)]
          (if-not (nil? rel-p2-pos) ;
            [rel-p2-pos p2o] ;(set p2-rel-p1)) rel-p2-pos]))
            (recur (inc i))))))))

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

(println (nth probes 2))
; (find-match (nth probes 4) (concat (take 4 probes) (drop 5 probes)))
(test-overlap (nth probes 4) (nth probes 1))
; (get-all-rotations [[1 2 3] [4 5 6]] 2)
; (defn rot [[x y z]] [(- z) x (- y)])
; (println (map rot (nth probes 4)))
; (test-orientation (nth probes 1) (map rot (nth probes 4)))
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;
