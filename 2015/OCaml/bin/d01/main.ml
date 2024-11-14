let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

let map_floor c =
    match c with
    | '(' -> 1
    | ')' -> -1
    | _ -> 0

let to_chars s =
    List.init (String.length s) (String.get s)

let do_p1 s =
    to_chars s
    |> List.map map_floor
    |> List.fold_left (+) 0

let do_p2 s =
    to_chars s
    |> List.map map_floor
    |> List.fold_left (fun (idx, (flr, found)) adj -> match found with
        | true -> (idx, (-1, true))
        | false -> if flr == -1
            then (idx, (-1, true))
            else (idx + 1, (flr + adj, false))
    ) (0, (0, false))
    |> fst

let () =
    let input = read_file "inputs/d01.txt" in
    Printf.printf "%d\n" (do_p1 input);
    Printf.printf "%d\n" (do_p2 input);
