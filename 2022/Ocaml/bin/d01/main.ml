let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

let get_packs s = Str.split (Str.regexp "\n\n") s

let get_packed_cals p =
    List.map (fun c -> int_of_string c) (Str.split (Str.regexp "\n") p)

let summed_packs s =
    let packs = get_packs s in
    let cals = List.map (fun p -> get_packed_cals p) packs in
    List.map (fun p -> List.fold_left (+) 0 p) cals

let do_p1 s = 
    List.fold_left (fun a x -> max a x) 0 (summed_packs s)

let do_p2 s =
    let sorted = List.sort (fun a b -> compare b a) (summed_packs s) in
    (List.nth sorted 0) + (List.nth sorted 1) + (List.nth sorted 2)

let () =
    let input = read_file "inputs/d01.txt" in
    Printf.printf "D01P01: %d\n" (do_p1 input);
    Printf.printf "D01P02: %d\n" (do_p2 input)
