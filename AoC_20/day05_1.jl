inpt = readlines("aoc_2020\\day05_input.txt")

seats = sort(parse.(Int, replace.(inpt, r"[FBRL]" => s -> occursin(s, "RB") ? 1 : 0), base = 2))
seats = sort(parse.(Int, replace.(replace.(inpt, r"R|B" => "1"), r"L|F" => "0"), base = 2))
# seats = sort(parse.(Int, join.([occursin(c, "RB") ? 1 : 0 for c in w] for w in inpt), base = 2))

println.((id+seats[1]) for id in 0:length(seats)-1 if (seats[id+1] != id+seats[1] && seats[id] != seats[id+1]-1))
