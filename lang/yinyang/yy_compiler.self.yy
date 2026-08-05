;; 囡囧共生 阴阳三进制 · 自举编译器源
;; YY Compiler · Self-Host Source
;;
;; 这是用 YinYang 语法编写的编译器本身::编译器编译自己的编译器
;;
;; 自举路径 (bootstrap path):
;;   Stage 0: yy_compiler.js (人工编写的 JS 编译器) 解析本文件
;;   Stage 1: 产出 yy_compiler.auto.js
;;   Stage 2: yy_compiler.auto.js 再次编译本文件 → 应产出相同结果
;;   若 Stage 1 == Stage 2, 则自举成功 = 编译器能编译自身
;;
;; 哲学意义: 这就是 autoclaw:用自己写的编程语言写自己的编译器
;;
;; ----------------------------------------------------------------
;; 数据结构: 符号表 / 代码生成 dispatch / 目标语言片段
;; ----------------------------------------------------------------

;; === 类型标记 ===
(= number   "YY.mkNum")
(= symbol   "SYMBOL")
(= string   "JSON.stringify")
(= container "YY.mkCell")
(= lambda    "YY.cellAppend")

;; === 代码生成入口 ===
;; codegen(ast):
;;   对 AST 中的每个 expr,根据其类型分派到具体生成器

(= codegen
  (λ (ast)
    (select (type ast)
      (number   (emit-num (val ast)))
      (string   (emit-str (val ast)))
      (symbol   (emit-sym (val ast)))
      (container (emit-container ast))
      (default  "null"))))

;; === 各类型生成器 ===
(= emit-num
  (λ (v) (++ "YY.mkNum(" (stringify v) ")")))

(= emit-str
  (λ (v) (++ "\"" v "\"")))

(= emit-sym
  (λ (v) v))

(= emit-container
  (λ (ast)
    (let head (head ast))
    (let ident (if (num? head)
                   (if (== head 0) "YY.mkCell(null,0)" "YY.mkCell(null,1)")
                   head))
    (let body (map codegen (tail ast)))
    (let body-js (join (map (λ (b) (++ "YY.cellAppend(c," b ";")) body)
                        ""))
    (if (mem? head '(评 判 变 print def λ := if))
        ;; 容器内联操作
        (emit-op head body)
        ;; 默认: 构建容器并填充
        (++ "(()=>{const c=" ident ";" body-js "return c;})()")))))

;; === 操作符分派 ===
(= emit-op
  (λ (op args)
    (select op
      (评 (++ "YY.opJudge(null,(" (nth 0 args) "))"))
      (判 (++ "JSON.stringify(" (nth 0 args) ")"))
      (变 (++ "YY.opMirror(null,(" (nth 0 args) "))"))
      (print (++ "console.log(" (join args ",") ")"))
      (def (++ "const " (nth 0 args) "=" (nth 1 args)))
      (λ  (++ "function(" (join (nth 0 args) ",") "){return " (nth 1 args) ";}"))
      (:= (++ (nth 0 args) "=" (nth 1 args)))
      (if (emit-if (nth 0 args) (nth 1 args) (nth 2 args)))
      (default (join (cons op args) " ")))))

(= emit-if
  (λ (cond then else)
    (++ "(" cond "?" then ":" else ")")))

;; === 顶层壳 ===
(= compile-src
  (λ (src)
    (let tokens '())
    (let ast (parse src))
    (let js-lines (map codegen ast))
    (join js-lines "\n")))

(= compile-file
  (λ (filename)
    (let src (read-file filename))
    (let js (compile-src src))
    (write-file (++ filename ".auto.js") js)))

(= main
  (λ ()
    (let args (command-line-args))
    (if (nil? args)
        (interactive-repl)
        (compile-file (nth 0 args)))))

(main)
