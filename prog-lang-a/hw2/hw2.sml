(* Dan Grossman, Coursera PL, HW2 Provided Code *)

(* if you use this function to compare two strings (returns true if the same
   string), then you avoid several of the functions in problem 1 having
   polymorphic types that may be confusing *)
fun same_string(s1 : string, s2 : string) =
    s1 = s2

(* put your solutions for problem 1 here *)
fun all_except_option(s1 : string, lst : string list) =
  let fun all_except_option(s1, lst, acc) =
    case lst of
      [] => NONE
    | x::xs => if same_string(s1, x)
               then SOME (acc @ xs)
               else all_except_option(s1, xs, x::acc)
  in
    all_except_option(s1, lst, [])
  end

fun get_substitutions1(lst : string list list, s : string) =
  case lst of
       [] => []
     | x::xs => let val subs = all_except_option(s, x)
                in 
                  case subs of
                       NONE => []
                     | SOME ys => ys @ get_substitutions1(xs, s)
                end

fun get_substitutions2(lst : string list list, s : string) =
  let fun get_substitutions(lst, s, acc) = 
      case lst of
           [] => acc
         | x::xs => let val subs = all_except_option(s, x)
                    in
                      case subs of 
                           NONE => []
                         | SOME ys => get_substitutions(xs, s, ys @ acc)
                      end
  in
    get_substitutions(lst, s, [])
  end


(* you may assume that Num is always used with values 2, 3, ..., 10
   though it will not really come up *)
datatype suit = Clubs | Diamonds | Hearts | Spades
datatype rank = Jack | Queen | King | Ace | Num of int 
type card = suit * rank

datatype color = Red | Black
datatype move = Discard of card | Draw 

exception IllegalMove

(* put your solutions for problem 2 here *)
