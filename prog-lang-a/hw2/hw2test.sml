(* Homework2 Simple Test *)
(* These are basic test cases. Passing these tests does not guarantee that your code will pass the actual homework grader *)
(* To run the test, add a new line to the top of this file: use "homeworkname.sml"; *)
(* All the tests should evaluate to true. For example, the REPL should say: val test1 = true : bool *)

val test1 = all_except_option ("string", ["string"]) = SOME []

val test2 = get_substitutions1 ([["foo"],["there"]], "foo") = []

val test3 = get_substitutions2 ([["foo"],["there"]], "foo") = []

val test4 = similar_names ([["Fred","Fredrick"],["Elizabeth","Betty"],["Freddie","Fred","F"]], {first="Fred", middle="W", last="Smith"}) =
	    [{first="Fred", last="Smith", middle="W"}, {first="Fredrick", last="Smith", middle="W"},
	     {first="Freddie", last="Smith", middle="W"}, {first="F", last="Smith", middle="W"}]

val test5 = card_color (Clubs, Num 2) = Black

val test6 = card_value (Clubs, Num 2) = 2

val test7 = remove_card ([(Hearts, Ace)], (Hearts, Ace), IllegalMove) = []

val test8 = all_same_color [(Hearts, Ace), (Hearts, Ace)] = true

val test9 = sum_cards [(Clubs, Num 2),(Clubs, Num 2)] = 4

val test10 = score ([(Hearts, Num 2),(Clubs, Num 4)],10) = 4

val test11 = officiate ([(Hearts, Num 2),(Clubs, Num 4)],[Draw], 15) = 6

val test12 = officiate ([(Clubs,Ace),(Spades,Ace),(Clubs,Ace),(Spades,Ace)],[Draw,Draw,Draw,Draw,Draw],42)
             = 3
val test13 = ((officiate([(Clubs,Jack),(Spades,Num(8))],
                         [Draw,Discard(Hearts,Jack)],
                         42);
               false) 
              handle IllegalMove => true)

val test14 = all_except_option("string", ["string1", "string", "string2"]) = SOME ["string1", "string2"]
val test15 = all_except_option("Fred", ["Fred","Fredrick"]) = SOME ["Fredrick"]
val test16 = all_except_option("Fred", ["Freddie","Fred","F"]) = SOME ["Freddie", "F"]

val test17 = get_substitutions1([["Fred","Fredrick"],["Elizabeth","Betty"],["Freddie","Fred","F"]],"Fred") = ["Fredrick","Freddie","F"]
val test18 = get_substitutions2([["Fred","Fredrick"],["Elizabeth","Betty"],["Freddie","Fred","F"]],"Fred") = ["Fredrick","Freddie","F"]
val test19 = card_value(Clubs, Ace) = 11
val test20 = ((remove_card([(Hearts, Ace)], (Clubs, Ace), IllegalMove); false)
                  handle IllegalMove => true)
val test21 = all_same_color([(Clubs, Ace), (Clubs, Ace)]) = true
val test22 = all_same_color([(Clubs, Ace), (Hearts, Ace)]) = false
val test23 = all_same_color([(Clubs, Ace), (Spades, Ace)]) =  true
val test24 = score([(Clubs, Ace)], 42) = 15
val test25 = score([(Clubs, Ace), (Spades, Ace)], 42) = 10
val test26 = score([(Clubs, Ace), (Spades, Ace), (Clubs, Ace)], 42) = 4
val test27 = score([(Clubs, Ace), (Spades, Ace), (Clubs, Ace), (Spades, Ace)],
42) = 3
val test28 = all_same_color([(Clubs, Ace), (Spades, Ace), (Clubs, Ace)]) = true
val test29 = all_same_color([(Clubs, Ace), (Spades, Ace), (Clubs, Ace), (Spades,
                            Ace)]) = true
val test30 = sum_cards([(Clubs, Ace)]) = 11
val test31 = sum_cards([(Clubs, Ace), (Spades, Ace)]) = 22
val test32 = sum_cards([(Clubs, Ace), (Spades, Ace), (Clubs, Ace)]) = 33
val test33 = sum_cards([(Clubs, Ace), (Spades, Ace), (Clubs, Ace), (Spades,
Ace)]) = 44
