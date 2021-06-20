(* Homework1 Simple Test *)
(* These are basic test cases. Passing these tests does not guarantee that your code will pass the actual homework grader *)
(* To run the test, add a new line to the top of this file: use "homeworkname.sml"; *)
(* All the tests should evaluate to true. For example, the REPL should say: val test1 = true : bool *)

val test1 = is_older ((1,2,3),(2,3,4)) = true

val test2 = number_in_month ([(2012,2,28),(2013,12,1)],2) = 1

val test3 = number_in_months ([(2012,2,28),(2013,12,1),(2011,3,31),(2011,4,28)],[2,3,4]) = 3

val test4 = dates_in_month ([(2012,2,28),(2013,12,1)],2) = [(2012,2,28)]

val test5 = dates_in_months ([(2012,2,28),(2013,12,1),(2011,3,31),(2011,4,28)],[2,3,4]) = [(2012,2,28),(2011,3,31),(2011,4,28)]

val test6 = get_nth (["hi", "there", "how", "are", "you"], 2) = "there"

val test7 = date_to_string (2013, 6, 1) = "June 1, 2013"

val test8 = number_before_reaching_sum (10, [1,2,3,4,5]) = 3

val test9 = what_month 70 = 3

val test10 = month_range (31, 34) = [1,2,2,2]

val test11 = oldest([(2012,2,28),(2011,3,31),(2011,4,28)]) = SOME (2011,3,31)

val test12 = number_in_months_challenge([(2012,2,28),(2013,12,1),(2011,3,31),(2011,4,28)], [2,2,3,4]) = 3

val test13 = dates_in_months_challenge([(2012,2,28),(2013,12,1),(2011,3,31),(2011,4,28)],[2,2, 3,4]) = [(2012,2,28),(2011,3,31),(2011,4,28)]

val test14 = is_leap_year(2100) = false
val test15 = is_leap_year(2104) = true
val test16 = is_leap_year(2400) = true

val test17 = reasonable_date((2021, 2, 29)) = false
val test18 = reasonable_date((2020, 2, 29)) = true
val test19 = reasonable_date((2100, 2, 29)) = false
val test20 = reasonable_date((2400, 2, 29)) = true

val test21 = is_older ((1, 2, 25),(1, 2, 25)) = false
val test22 = oldest([(5,12,15),(5,12,10),(5,12,1)]) = SOME (5,12,1)
val test23 = date_to_string((1,2,25)) = "February 25, 1"
val test24 = date_to_string((6,2,1)) = "February 1, 6"
val test25 = number_before_reaching_sum(1, [2]) = 0
val test26 = number_before_reaching_sum(5, [3,1,2]) = 2
val test27 = number_before_reaching_sum(5, [3,2,2]) = 1
val test28 = number_before_reaching_sum(6, [4,1,1,1]) = 2
