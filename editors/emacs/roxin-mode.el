;;; roxin-mode.el --- Major mode for Roxin models -*- lexical-binding: t; -*-

;; Version: 0.0.1
;; Package-Requires: ((emacs "29.1"))
;; Keywords: languages

;;; Commentary:

;; Syntax highlighting for Roxin (.rox) files.
;; The Roxin syntax is provisional; update the keyword lists below as it changes.
;;
;; Install: put this file on your `load-path' and add to your init file:
;;   (require 'roxin-mode)

;;; Code:

(defgroup roxin nil
  "Major mode for Roxin models."
  :group 'languages
  :prefix "roxin-")

(defvar roxin-mode-syntax-table
  (let ((table (make-syntax-table)))
    ;; "--" starts a comment that runs to the end of the line.
    (modify-syntax-entry ?- ". 12" table)
    (modify-syntax-entry ?\n ">" table)
    (modify-syntax-entry ?\" "\"" table)
    (modify-syntax-entry ?\\ "\\" table)
    (modify-syntax-entry ?_ "_" table)
    (dolist (c '(?+ ?* ?/ ?% ?= ?< ?> ?^ ?! ?| ?. ?, ?:))
      (modify-syntax-entry c "." table))
    (modify-syntax-entry ?\( "()" table)
    (modify-syntax-entry ?\) ")(" table)
    (modify-syntax-entry ?\[ "(]" table)
    (modify-syntax-entry ?\] ")[" table)
    (modify-syntax-entry ?\{ "(}" table)
    (modify-syntax-entry ?\} "){" table)
    table)
  "Syntax table for `roxin-mode'.")

(defvar roxin-font-lock-keywords
  (list
   ;; Declarations that introduce a name.
   `(,(rx symbol-start (group (or "state" "type" "step" "system"))
          (+ space) (group (+ (any alnum ?_))))
     (1 font-lock-keyword-face) (2 font-lock-type-face))
   `(,(rx symbol-start (group (or "invariant" "metric" "extern"))
          (+ space) (group (+ (any alnum ?_))))
     (1 font-lock-keyword-face) (2 font-lock-function-name-face))
   `(,(rx symbol-start (group (or "param" "data"))
          (+ space) (group (+ (any alnum ?_))))
     (1 font-lock-keyword-face) (2 font-lock-variable-name-face))

   ;; Multi-word keywords.
   `(,(rx symbol-start
          (or (seq "input" (+ space) "map")
              (seq "record" (+ space) "substeps")
              (seq "keep" (+ space) "running"))
          symbol-end)
     . font-lock-keyword-face)

   ;; Step sections: input: allowed: mechanism:
   `(,(rx symbol-start (group (or "input" "allowed" "mechanism")) symbol-end
          (* space) ":")
     (1 font-lock-keyword-face))

   ;; Remaining keywords.
   `(,(rx symbol-start
          (or "state" "type" "system" "invariant" "metric" "param" "data"
              "tolerance" "extern"
              "run" "from" "for" "steps" "sweep" "until" "every"
              "if" "then" "else" "with" "within" "in"
              "and" "or" "not")
          symbol-end)
     . font-lock-keyword-face)

   ;; `step' used as a value (the current step number).
   `(,(rx symbol-start "step" symbol-end) . font-lock-builtin-face)

   ;; Primitive types.
   `(,(rx symbol-start (or "int" "float" "decimal" "bool" "text") symbol-end)
     . font-lock-type-face)

   ;; Constants and rounding modes.
   `(,(rx symbol-start (or "true" "false" "down" "up" "nearest") symbol-end)
     . font-lock-constant-face)

   ;; Built-in functions.
   `(,(rx symbol-start
          (group (or "sign" "abs" "min" "max" "round" "sqrt" "to_float" "sample"))
          symbol-end (* space) "(")
     (1 font-lock-builtin-face))

   ;; `s' and `u' (old state and input record), and the spread `..s'.
   `(,(rx ".." (group "s") symbol-end) (1 font-lock-variable-name-face))
   `(,(rx symbol-start (group (any "su")) "." (any alpha ?_))
     (1 font-lock-variable-name-face))
   `(,(rx symbol-start (group "s") (+ space) "with" symbol-end)
     (1 font-lock-variable-name-face))

   ;; Field access: s.temp, u.heat, record.field
   `(,(rx (any alnum ?_ ?\) ?\]) "." (group (any alpha ?_) (* (any alnum ?_))))
     (1 'font-lock-property-use-face))

   ;; Numbers, including percent literals such as 0.0001%.
   `(,(rx symbol-start (group (+ digit) (? "." (+ digit))
                              (? (any "eE") (? (any "+-")) (+ digit))
                              "%")
          (or eol (not (any alnum ?_ ?\())))
     (1 'font-lock-number-face))
   `(,(rx symbol-start (+ digit) (? "." (+ digit))
          (? (any "eE") (? (any "+-")) (+ digit)))
     (0 'font-lock-number-face))

   ;; Capitalized names (types, steps, systems, choices).
   `(,(rx symbol-start (any "A-Z") (* (any alnum ?_)) symbol-end)
     . font-lock-type-face)

   ;; Operators.
   `(,(rx (or ">>" "->" ".." "==" "!=" "<=" ">=" "//"
              "<" ">" "+" "-" "*" "%" "^" "="))
     (0 'font-lock-operator-face)))
  "Font-lock rules for `roxin-mode'.")

;;;###autoload
(define-derived-mode roxin-mode prog-mode "Roxin"
  "Major mode for editing Roxin models."
  (setq-local comment-start "-- ")
  (setq-local comment-end "")
  (setq-local comment-start-skip "--+[ \t]*")
  (setq-local indent-tabs-mode nil)
  (setq-local tab-width 4)
  (setq-local font-lock-defaults '(roxin-font-lock-keywords)))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.rox\\'" . roxin-mode))

(provide 'roxin-mode)

;;; roxin-mode.el ends here
