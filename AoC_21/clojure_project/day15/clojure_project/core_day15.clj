(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]
            [clojure.data.priority-map :refer [priority-map]]))

(get-input "15" inpt)
(get-test "15" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(def grid (to-int-grid inpt))
(def h (count grid))
(def w (count (first grid)))

(def costs
  (apply hash-map
    (apply concat
      (for [y (range h) x (range w)]
        [[x y] (nth2d grid x y)]))))

(def costs2
  (apply hash-map
    (apply concat
      (for [y (range (* 5 h)) x (range (* 5 w))
            :let [to-add (+ (quot x w) (quot y h))]]
        [[x y] (-> (nth2d grid (mod x w) (mod y h))
                   (+ to-add) dec (mod 9) inc)]))))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defn neighbors
  "Returns n's neighbors"
  [g n] (let [x (first n) y (last n)]
          (select-keys g
            #{[(dec x) y] [(inc x) y] [x (dec y)] [x (inc y)]})))

(defn map-vals [m f]
  (into {} (for [[k v] m] [k (f v)])))

(defn remove-keys [m pred]
  (select-keys m (filter (complement pred) (keys m))))

(defn dijkstra
  [start g f]
  (loop [q (priority-map start 0) r {}]
    (if-let [[v d] (peek q)]
      (let [dist (-> (f g v) (remove-keys r) (fmap (partial + d)))]
        (recur (merge-with min (pop q) dist) (assoc r v d)))
      r)))

;;;;;;;;;;;;;;
;;; Part01 ;;;
;;;;;;;;;;;;;;

(println ((dijkstra [0 0] costs neighbors) [99 99]))

;;;;;;;;;;;;;;
;;; Part02 ;;;
;;;;;;;;;;;;;;

(println (time ((dijkstra [0 0] costs2 neighbors) [499 499])))
