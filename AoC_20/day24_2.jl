inpt = readlines("AoC_20\\day24_input.txt")

blackTiles = fill(false, (200,200))
dirs = Dict(["e" => CartesianIndex(1,0), "w" => CartesianIndex(-1,0), "se" => CartesianIndex(0,1), "sw" => CartesianIndex(-1,1), "ne" => CartesianIndex(1,-1), "nw" => CartesianIndex(0,-1)])

for i in 1:length(inpt)
    pos = CartesianIndex(100,100)
    while length(inpt[i]) > 0
        if inpt[i][1] == 's' || inpt[i][1] == 'n'
            pos += dirs[inpt[i][1:2]]
            inpt[i] = inpt[i][3:end]
        else
            pos += dirs[inpt[i][1:1]]
            inpt[i] = inpt[i][2:end]
        end
    end
    blackTiles[pos] = !blackTiles[pos]
    # else push!(blackTiles, pos) end
end
summ = sum(blackTiles)
# Part 2

for j in 1:100
    bT = deepcopy(blackTiles)
    for i in CartesianIndex(2,2):CartesianIndex(size(blackTiles))-CartesianIndex(1,1)
        c = 0
        for d in values(dirs)
            c += blackTiles[i+d]
        end
        if blackTiles[i]
            bT[i] = c >= 1 && c <= 2
        else bT[i] = c == 2 end
    end
    # println("$j: ", sum(bT))
    global blackTiles = bT
end
