(* Dan Grossman, Coursera PL, HW2 Provided Code *)

(* if you use this function to compare two strings (returns true if the same
   string), then you avoid several of the functions in problem 1 having
   polymorphic types that may be confusing *)
fun same_string(s1 : string, s2 : string) =
    s1 = s2

(* put your solutions for problem 1 here *)
(* 1. a. *)
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

(* 1. b. *)
fun get_substitutions1(lst : string list list, s : string) =
  case lst of
       [] => []
     | x::xs => case all_except_option(s, x) of
                       NONE => get_substitutions1(xs, s)
                     | SOME ys => ys @ get_substitutions1(xs, s)

(* 1. c. *)
fun get_substitutions2(lst : string list list, s : string) =
  let fun get_substitutions(lst, s, acc) = 
      case lst of
           [] => acc
         | x::xs => let 
                      val subs = case all_except_option(s, x) of 
                           NONE => []
                         | SOME ys => ys
                    in
                      get_substitutions(xs, s, acc @ subs)
                    end
  in
    get_substitutions(lst, s, [])
  end

(* 1. d. *)
fun similar_names(names : string list list, name : {first: string, middle: string, last: string}) = 
  let 
    val {first, middle, last} = name
    val subs = get_substitutions2(names, first)

    fun lst_to_data_type(names, acc) = 
      case names of
           [] => acc
         | x::xs => lst_to_data_type(xs, acc @ [{first=x, middle=middle,
         last=last}])
  in
    name :: lst_to_data_type(subs, [])
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

(* 2. a. *)
fun card_color card =
  case card of
       (Clubs, _) => Black
     | (Spades, _) => Black
     | (Diamonds, _) => Red
     | (Hearts, _) => Red

(* 2. b. *)
fun card_value card =
  case card of
       (_, Num n) => n
     | (_, Ace) => 11
     | (_, _) => 10

(* 2. c. *)
fun remove_card (cs, c, e) =
  let 
    fun remove_card (cs, acc) =
      case cs of
           [] => raise e
         | x::xs => if x = c
                    then acc @ xs
                    else remove_card(xs, x::acc)

    val acc = remove_card(cs, [])
  in
    if acc = cs
    then raise e
    else acc
  end

(* 2. d. *)
fun all_same_color cards =
  case cards of
    [] => true
  | _::[] => true
  | head::(neck::tl) => card_color(head) = card_color(neck) andalso
  all_same_color((neck::tl))

(* 2. e. *)
fun sum_cards cards =
  let
    fun sum_cards (cards, sum) =
      case cards of
           [] => sum
         | x::xs => sum_cards(xs, sum + card_value(x))
  in
    sum_cards(cards, 0)
  end

(* 2. f. *)
fun score(cards, goal) =
  let
    val sum = sum_cards(cards)
    val prelim_score = if sum > goal
                       then 3 * (sum - goal)
                       else goal - sum
  in
    if all_same_color(cards)
    then prelim_score div 2
    else prelim_score
  end

(* 2. g. *)
fun officiate(cards, moves, goal) =
  let
    fun move(hand, pool, moves) =
      case moves of
           [] => score(hand, goal)
         | (Discard card)::xs => move(remove_card(hand, card, IllegalMove), pool, xs)
         | Draw::xs => case pool of
                            [] => score(hand, goal)
                          | y::ys =>
                              let
                                val new_hand = y::hand
                                val new_score = score(new_hand, goal)
                              in
                                if new_score > goal
                                then new_score
                                else move(new_hand, ys, xs)
                              end
  in
    move([], cards, moves)
  end
