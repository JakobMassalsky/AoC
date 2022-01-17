inpt = readlines("AoC_20\\day24_input.txt")

blackTiles = Set{Vector{Int}}()
dirs = Dict(["e" => [1,0], "w" => [-1,0], "se" => [0,1], "sw" => [-1,1], "ne" => [1,-1], "nw" => [0,-1]])

for i in 1:length(inpt)
    pos = [0,0]
    while length(inpt[i]) > 0
        if inpt[i][1] == 's' || inpt[i][1] == 'n'
            pos += dirs[inpt[i][1:2]]
            inpt[i] = inpt[i][3:end]
        else
            pos += dirs[inpt[i][1:1]]
            inpt[i] = inpt[i][2:end]
        end
    end
    if pos in blackTiles pop!(blackTiles, pos)
    else push!(blackTiles, pos) end
end

# Part 2
