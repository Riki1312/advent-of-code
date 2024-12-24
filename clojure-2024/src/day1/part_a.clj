(ns day1.part-a
  (:require
   [clojure.string :as str]
   [clojure.test :as test]))

;; Solution
(defn diff-of-sorted-elements [l1 l2]
  (let [l1 (sort l1)
        l2 (sort l2)
        ld (map #(abs (- %1 %2)) l1 l2)
        d (reduce + ld)]
    d))

(defn parse-line [line]
  (map (fn [s] (Integer/parseInt s)) (str/split line #" +")))

(defn parse-lines [lines]
  (let [pairs (map parse-line lines)
        l1 (map first pairs)
        l2 (map second pairs)]
    [l1 l2]))

(defn parse-file [filename]
  (let [lines (slurp filename)]
    (parse-lines (str/split-lines lines))))

(defn solve [filename]
  (let [[l1 l2] (parse-file filename)]
    (diff-of-sorted-elements l1 l2)))

;; Tests
(test/deftest parse-lines-test
  (test/testing "parsing multiple lines"
    (let [input ["3   4"
                 "4   3"
                 "2   5"]
          expected-l1 [3 4 2]
          expected-l2 [4 3 5]]
      (test/is (= [expected-l1 expected-l2] (parse-lines input))))))

(test/deftest diff-of-sorted-elements-test
  (test/testing "calculating difference of sorted elements"
    (let [l1 [3 4 2 1 3 3]
          l2 [4 3 5 3 9 3]
          expected 11]
      (test/is (= expected (diff-of-sorted-elements l1 l2))))))

;; Run all tests
(defn run-tests []
  (test/run-tests 'day1.part-a))

(comment

  (run-tests)

  (solve (str (System/getProperty "user.dir") "/src/day1/input_a.txt"))

  :rcf)
