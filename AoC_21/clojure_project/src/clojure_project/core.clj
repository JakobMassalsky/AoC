(ns clojure-project.core
  (:gen-class
   :require [clojure.string :as string]))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))

(defn sublist [list begin end]
  (let [[b s] (if (> end begin) [end begin] [begin end])]
    (take (- b s) (drop s list))))

(defn in?
  "true if coll contains elm"
  [coll elm]
  (some #(= elm %) coll))

(defn not-in?
  "true if coll contains elm"
  [coll elm]
  (not (some #(= elm %) coll)))

(defn to-int
  [stri]
  (Integer/parseInt stri))

(defmacro defs
  "assign multiple symbols"
  [& bindings]
  {:pre [(even? (count bindings))]}
  `(do
    ~@(for [[symb val] (partition 2 bindings)]
       `(def ~symb ~val))))

(defmacro ++
  "Increases in place"
  [symb]
  `(let [x# (inc ~symb)]
    (def ~symb x#)
    ~symb))

(defmacro --
  "decreases in place"
  [symb]
  `(let [x# (dec ~symb)]
    (def ~symb x#)
    ~symb))

(defmacro +=
  "adds in place"
  [symb incr]
  `(let [x# (+ ~symb ~incr)]
    (def ~symb x#)
    ~symb))

(defmacro -=
  "subtracts in place"
  [symb incr]
  `(let [x# (- ~symb ~incr)]
    (def ~symb x#)
    ~symb))

(defmacro get-input
  "inits Day x"
  [day name]
  `(let [d# ~day]
    (def ~name (slurp (clojure.string/join "" ["C:/Users/jakob/OneDrive/Desktop/Coding/AoC/AoC_21/clojure_project/day" d# "/clojure_project/input.txt"])))))

(defmacro get-test
  "inits Day x"
  [day name]
  `(let [d# ~day]
    (def ~name (slurp (clojure.string/join "" ["C:/Users/jakob/OneDrive/Desktop/Coding/AoC/AoC_21/clojure_project/day" d# "/clojure_project/input_test.txt"])))))


(defmacro equal-macro
  "Define an x= macro"
  [nam func]
  `(defn ~(symbol nam) [symb# incr#]
     (let [x# (~func symb# incr#)]
       (def symb# x#)
       x#)))

(defn sign
  "Returns 1 * sign of x, nil if x is 0"
  [x]
  (if (zero? x)
    nil
    (/ x (Math/abs x))))

(defn to-range-inc
  "Range from p1 to p2 inclusive, repeat p1 if p1 = p2"
  [p1 p2]
  (if-let [d (sign (- p2 p1))]
    (range p1 (+ p2 d) d)
    (repeat p1)))

(defn to-int-grid
  "2d string grid to 2d int grid"
  [inpt]
  (->> inpt clojure.string/split-lines
       (map #(clojure.string/split % #""))
       (map #(map to-int %))))

(defn print2d [coll]
  (let [h (count coll)
        w (count (first coll))]
    (dotimes [l h]
      (println (nth coll l)))))

(defn count-n
  [coll n]
  (count (filter #{n} (flatten coll))))

(defn nth2d
  [coll x y]
  (if (and (and (>= x 0) (< x (count (first coll))))
           (and (>= y 0) (< y (count coll))))
    (nth (nth coll y) x)
    0))

(defn nth2d-bounds
  ([coll x y w h]
   (if (and (and (>= x 0) (< x w))
            (and (>= y 0) (< y h)))
     (nth (nth coll y) x)
     0))
  ([coll x y w h d]
   (if (and (and (>= x 0) (< x w))
            (and (>= y 0) (< y h)))
     (nth (nth coll y) x)
     d)))

; (defn cart [colls]
;   "Compute the cartesian product of list of lists"
;   (if (empty? colls)
;     '(())
;     (for [more (cart (rest colls))
;           x (first colls)]
;       (cons x more))))

(def first? #(if (empty? %) nil (first %)))

(defn cart [& colls]
  "Compute the cartesian product of lists"
  (if (empty? colls) '(())
    (for [more (apply cart (rest colls))
          x (first colls)]
      (cons x more))))

(def >> bit-shift-right)
(def << bit-shift-right)

(def sum-to-n #(>> (* % (inc %)) 1))

(defn apply-till-equal [f s]
  (loop [sn s]
    (let [sn' (f sn)]
      (if (= sn' sn) sn
        (recur sn')))))

(defn fmap [m f]
  (into {} (for [[k v] m] [k (f v)])))

(def memusage (float (/ (- (-> (java.lang.Runtime/getRuntime) (.totalMemory)) (-> (java.lang.Runtime/getRuntime) (.freeMemory))) 1024)))
