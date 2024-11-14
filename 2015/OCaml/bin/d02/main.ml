let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

type dims = {
    x : int;
    y : int;
    z : int;
}

let lines s = Str.split (Str.regexp "\n") s

let to_dims s =
    let parts = Str.split (Str.regexp "x") s
        |> List.map int_of_string
        |> List.sort compare in
    { x = List.nth parts 0; y = List.nth parts 1; z = List.nth parts 2 }

let to_dims_list s =
    lines s
    |> List.map to_dims

let do_p1 s =
    to_dims_list s
    |> List.map (fun ({x; y; z}) -> 2*x*y + 2*x*z + 2*y*z + x*y)
    |> List.fold_left (+) 0

let do_p2 s =
    to_dims_list s
    |> List.map (fun {x; y; z} -> x+x+y+y+ x*y*z)
    |> List.fold_left (+) 0

let () =
    let input = read_file "inputs/d02.txt" in
    Printf.printf "D02P01: %d\n" (do_p1 input);
    Printf.printf "D02P02: %d\n" (do_p2 input)
