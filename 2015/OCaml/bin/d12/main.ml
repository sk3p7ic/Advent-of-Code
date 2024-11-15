let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

let get_nums s =
    Str.split (Str.regexp "[^-0-9]") s
    |> List.filter_map (fun v -> int_of_string_opt v)

let do_p1 s =
    get_nums s
    |> List.fold_left (+) 0

let () =
    let input = read_file "inputs/d12.txt" in
    Printf.printf "D12P01: %d\n" (do_p1 input);
