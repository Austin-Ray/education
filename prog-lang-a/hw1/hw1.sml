fun is_older(date1 : (int * int * int), date2 : (int * int * int)) =
  let
    val d1y = #1 date1
    val d1m = #2 date1
    val d1d = #3 date1

    val d2y = #1 date2
    val d2m = #2 date2
    val d2d = #3 date2

    val same_year = d1y = d2y;
    val same_month = d1m = d2m;
  in
    (d1y < d2y)
    orelse (same_year andalso (d1m < d2m))
    orelse (same_year andalso same_month andalso (d1d < d2d))
  end

fun in_month(date : (int * int * int), month : int) = #2 date = month

fun number_in_month(dates : (int * int * int) list, month : int) =
  let 
    fun count_date(date : (int * int * int), month : int) =
      if in_month(date, month) 
      then 1
      else 0
  in
    if null dates
    then 0
    else count_date(hd dates, month) + number_in_month(tl dates, month) 
  end

fun number_in_months(dates : (int * int * int) list, months : int list) =
  if null months
  then 0
  else number_in_month(dates, hd months) + number_in_months(dates, tl months)

fun dates_in_month(dates : (int * int * int) list, month : int) =
  if null dates
  then []
  else if in_month(hd dates, month)
  then (hd dates) :: dates_in_month(tl dates, month)
  else dates_in_month(tl dates, month)

fun dates_in_months(dates : (int * int * int) list, months : int list) =
  if null months
  then []
  else dates_in_month(dates, hd months) @ dates_in_months(dates, tl months)

fun get_nth(xs : 'a list, n : int) =
  if n = 1
  then hd xs
  else get_nth(tl xs, n-1)

fun date_to_string(date : (int * int * int)) =
  let
    val months = [
      "January", "February", "March", "April", "May", "June", "July", "August",
      "September", "October", "November", "December"
    ]
    val year = Int.toString(#1 date)
    val month = get_nth(months, #2 date)
    val day = Int.toString(#3 date)
  in
    month ^ " " ^ day ^ ", " ^ year
  end

fun number_before_reaching_sum(sum : int, pos_ints : int list) =
  if null pos_ints orelse (sum - (hd pos_ints)) <= 0
  then 0
  else 1 + number_before_reaching_sum((sum - (hd pos_ints)), (tl pos_ints))

(* Also used for challenge problem *)
val last_day_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

fun what_month(day : int) =
  1 + number_before_reaching_sum(day, last_day_month)

fun month_range(day1 : int, day2 : int) = 
  if day1 > day2
  then []
  else what_month(day1) :: month_range(day1 + 1, day2)

fun oldest(dates : (int * int * int) list) =
  let
    fun oldest(date : (int * int * int), compare : (int * int * int) list) =
      if null compare then date
      else if is_older(date, hd compare)
      then oldest(date, tl compare)
      else oldest(hd compare, tl compare)
  in
    if null dates
    then NONE
    else SOME (oldest(hd dates, tl dates))
  end

(* Challenge problems *)
fun in_list(elem : int, cmp_list : int list) = 
  if null cmp_list
  then false
  else if elem = (hd cmp_list)
  then true
  else in_list(elem, tl cmp_list)

fun dedupe(dupes : int list) =
  if null dupes
  then []
  else if in_list(hd dupes, tl dupes)
  then dedupe(tl dupes)
  else hd dupes :: dedupe(tl dupes)

fun number_in_months_challenge(dates : (int * int * int) list, months : int list) =
  number_in_months(dates, dedupe(months))

fun dates_in_months_challenge(dates : (int * int * int) list, months : int list) =
  dates_in_months(dates, dedupe(months)) 

val last_day_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
val last_day_month_leap = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

fun good_year(year : int) = year > 0

fun good_month(month : int) = month > 0 andalso month < 13

fun is_leap_year(year : int) =
  year mod 4 = 0 andalso (year mod 100 <> 0 orelse year mod 400 = 0)

fun good_day(day : int, month : int, year : int) =
  day > 0 andalso 
  if is_leap_year(year)
  then day <= get_nth(last_day_month_leap, month)
  else day <= get_nth(last_day_month, month)

fun reasonable_date(date : int * int * int) =
  let
    val year = #1 date
    val month = #2 date
    val day = #3 date
  in
    good_year(year) andalso good_month(month) andalso good_day(day, month, year)
  end
