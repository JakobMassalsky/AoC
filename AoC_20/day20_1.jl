inpt = split.(split(read("AoC_20\\day20_input.txt", String), "\r\n\r\n"), "\r\n")
deleteat!(inpt[end], length(inpt[end]))
tiles = Dict{Int, Array{}}()

for i in inpt
    tmp = []
    push!(tmp, [i[2][x] == '#' for x in 1:10])
    push!(tmp, [i[end][x] == '#' for x in 1:10])
    push!(tmp, [i[x][1] == '#' for x in 2:11])
    push!(tmp, [i[x][end] == '#' for x in 2:11])
    # println(tmp)
    push!(tiles, parse(Int, i[1][end-5:end-1]) => tmp)
end
acc = 1
for k in keys(tiles)
    matches = 0
    for j in keys(tiles)
        if k != j
            for v in tiles[k]
                for w in tiles[j]
                    matches += v == w || v == reverse!(w)
                end
            end
        end
    end
    println(k, " ", matches)
    global acc *= matches == 2 ? k : 1
end
println(acc)
