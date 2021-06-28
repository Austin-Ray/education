(* Coursera Programming Languages, Homework 3, Provided Code *)

exception NoAnswer

datatype pattern = Wildcard
		 | Variable of string
		 | UnitP
		 | ConstP of int
		 | TupleP of pattern list
		 | ConstructorP of string * pattern

datatype valu = Const of int
	      | Unit
	      | Tuple of valu list
	      | Constructor of string * valu

fun g f1 f2 p =
    let 
	val r = g f1 f2 
    in
	case p of
	    Wildcard          => f1 ()
	  | Variable x        => f2 x
	  | TupleP ps         => List.foldl (fn (p,i) => (r p) + i) 0 ps
	  | ConstructorP(_,p) => r p
	  | _                 => 0
    end

(**** for the challenge problem only ****)

datatype typ = Anything
	     | UnitT
	     | IntT
	     | TupleT of typ list
	     | Datatype of string

(**** you can put all your code here ****)

(* define pipeline for improved readability *)
infix |>
fun x |> f = f x 

(* 1. *)
val only_capitals = List.filter (fn x => (x, 0) |> String.sub |> Char.isUpper)

(* 2. *)
fun longest_string1 str = List.foldl (fn (x, y) => if String.size x > String.size y
																							 then x else y) "" str

(* 3. *)
fun longest_string2 str = List.foldl (fn(x, y) => if String.size y > String.size x
																							then y else x) "" str

(* 4. *)
fun longest_string_helper f xs = List.foldl(fn(x, y) => if f(String.size x,
	String.size y) then x else y) "" xs

val longest_string3 = longest_string_helper (fn (x, y) => x > y)

val longest_string4 = longest_string_helper (fn (x, y) => x >= y)

(* 5. *)
val longest_capitalized = longest_string1 o only_capitals

(* 6. *)
val rev_string = String.implode o List.rev o String.explode

(* 7. *)
fun first_answer f xs =
	case xs of 
			[] => raise NoAnswer
		| x::xs => case f x of
									NONE => first_answer f xs
								| SOME v => v

(* 8. *)
fun all_answers f xs = 
	let fun helper(f, xs, acc) =
		case xs of
			 [] => SOME acc
		 | x::xs => case f x of
								 NONE => NONE
							 | SOME lst => helper(f, xs, lst @ acc) 
	in
		helper(f, xs, [])
	end

(* 9. a. *)
val count_wildcards = g (fn _ => 1) (fn _ => 0)

(* 9. b. *)
val count_wild_and_variable_lengths = g (fn _ => 1) (fn x => String.size x)

(* 9. c. *)
fun count_some_var(var, pat) = g (fn _ => 0) (fn x => if x = var then 1 else 0) pat

(* 10. *)
val check_pat = 
	let 
		fun collect_variables p =
			case p of 
				Variable x	=> [x]
			| TupleP ps		=> List.foldl (fn (p,i) => (collect_variables p) @ i) [] ps
			| ConstructorP(_, ps) => collect_variables ps
			| _ => []

		fun check_for_dupes lst = 
			case lst of 
				[] => true
			| x::xs => not (List.exists (fn j => x = j) xs) andalso check_for_dupes xs

	in
		check_for_dupes o collect_variables
	end

(* 11. *)
fun match valpat = 
	case valpat of
				(_, Wildcard) => SOME []
			| (v, Variable x) => SOME[(x, v)]
			| (Unit, UnitP) => SOME []
			| (Const x1, ConstP x2) => if x1 = x2 then SOME [] else NONE
			| (Constructor(s1, v), ConstructorP(s2, p)) => if s1 = s2 then match(v, p) else NONE
			| (Tuple vs, TupleP ps) => if List.length vs = List.length ps
															   then all_answers match (ListPair.zip(vs, ps))
																 else NONE
			| _ => NONE

(* 12. *)
fun first_match valu patterns =
		SOME (first_answer (fn x => match(valu, x)) patterns)
		handle NoAnswer => NONE
