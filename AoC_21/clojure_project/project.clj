(defproject clojure_project "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/data.priority-map "1.1.0"]
                 [org.clojure/clojure "1.10.3"]
                 [org.clojure/algo.generic "0.1.0"]]
  :main ^:skip-aot clojure-project.core
  :target-path "target/%s"
  :profiles { :uberjar {:aot :all
                        :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
