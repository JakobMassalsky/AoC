(ns clojure-project.core-aoc
  (:require [clojure-project.core :refer :all]
            [clojure.string :as string]
            [clojure.set :as set]))

(get-input "16" inpt)
(get-test "16" testinpt)

;;;;;;;;;;;;;;;;;;;;
;;; Inputparsing ;;;
;;;;;;;;;;;;;;;;;;;;

(defn hex-to-binary [s]
  (let [f (hash-map \0 "0000" \1 "0001" \2 "0010" \3 "0011"
                    \4 "0100" \5 "0101" \6 "0110" \7 "0111"
                    \8 "1000" \9 "1001" \A "1010" \B "1011"
                    \C "1100" \D "1101" \E "1110" \F "1111")]
    (apply str (map f s))))

(def bininpt (hex-to-binary (string/trim inpt)))

;;;;;;;;;;;;;;;;;
;;; functions ;;;
;;;;;;;;;;;;;;;;;

(defs parse-num-pkgs 0 parse-all-pkgs 0)
(def bin-to-int #(read-string (str "2r" %)))
(def ops [#(reduce + %) #(reduce * %)
          #(apply min %) #(apply max %) nil
          #(if (apply > %) 1 0)
          #(if (apply < %) 1 0)
          #(if (apply = %) 1 0)])

(defn literal-num [ps]
  (let [cnt (count (take-while #(= (first %) \1) (partition 5 ps)))
        res (->> ps (partition 5) (take (inc cnt)) (mapcat next)
              (apply str) bin-to-int)]
     [res (* (inc cnt) 5)]))

(defn parse-pkg [ps]
  (let [version (bin-to-int (subs ps 0 3))
        opid (bin-to-int (subs ps 3 6))]

    (if (= opid 4)
      (let [[res le] (literal-num (subs ps 6))]  ; Literal Number
        [version (+ le 6) res])

      (let [b (= (bin-to-int (subs ps 6 7)) 0)
            bit-len (bin-to-int (subs ps 7 22))
            numpkgs (bin-to-int (subs ps 7 18))
            [ver le val] (if b
                           (parse-pkgs (subs ps 22 (+ 22 bit-len)) -1)
                           (parse-pkgs (subs ps 18) numpkgs))]
        [(+ version ver)
         (if b (+ 22 bit-len) (+ 18 le))
         ((ops opid) val)]))))

(defn parse-pkgs [ps n]
  (loop [s ps i 0 sumlength 0 sumver 0 result []]
    (if (or (and (> n 0) (>= i n)) (< (count s) 11))
      [sumver sumlength (flatten result)]
      (let [[ver le res] (parse-pkg s)]
        (recur (subs s le) (inc i) (+ sumlength le)
               (+ sumver ver) (conj result res))))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Part01 ;;;;;; Part02 ;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(parse-pkg bininpt)
