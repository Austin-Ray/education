
#lang racket

(provide (all-defined-out)) ;; so we can put tests in a second file

;; put your code below

;; 1.
(define (sequence low high stride)
  (if (< (- high low) 0)
      null
      (cons low (sequence (+ low stride) high stride))))

;; 2.
(define (string-append-map xs suffix)
  (map (lambda (x) (string-append x suffix)) xs))

;; 3.
(define (list-nth-mod xs n)
  (cond [(< n 0) (error "list-nth-mod: negative number")]
        [(null? xs) (error "list-nth-mod: empty list")]
        [#t (car (list-tail xs (remainder n (length xs))))]))

;; 4.
(define (stream-for-n-steps s n)
  (if (= n 0)
      null
      (let ([next (s)])
        (cons (car next) (stream-for-n-steps (cdr next) (- n 1))))))

;; 5.
(define funny-number-stream
  (letrec ([f (lambda (x)
                (cons (if (= 0 (remainder x 5)) (- x) x)
                      (lambda () (f (+ x 1)))))])
    (lambda () (f 1))))

;; 6.
(define dan-then-dog
  (letrec ([dan (lambda () (cons "dan.jpg" dog))]
           [dog (lambda () (cons "dog.jpg" dan))])
  dan))


;; 7.
(define (stream-add-zero s)
    (lambda ()
      (let ([next (s)])
            (cons (cons 0 (car next)) (lambda () (stream-add-zero (cdr next)))))))

;; 8.
(define (cycle-lists xs ys)
  (letrec ([f (lambda(n)
                (cons (cons (list-nth-mod xs n) (list-nth-mod ys n)) ;; Pair
                 (lambda () (f (+ n 1)))))])
    (lambda () (f 0))))

;; 9.
(define (vector-assoc v vec)
  (letrec ([f (lambda (n)
               (cond [(>= n (vector-length vec)) #f]
                     [(and (cons? (vector-ref vec n)) (equal? (car (vector-ref vec n)) v)) (vector-ref vec n)]
                     [#t (f (+ n 1))]))])
    (f 0)))

;; 10.
(define (cached-assoc xs n)
  (letrec([cache (vector n #f)]
          [pos 0])
    (lambda (x)
      (let ([ans (vector-assoc x cache)])
        (or ans
            (let ([new-ans (assoc x xs)])
              (and new-ans
                   (begin
                     (vector-set! cache pos new-ans)
                     (set! pos (remainder (+ pos 1) n))
                     new-ans))))))))