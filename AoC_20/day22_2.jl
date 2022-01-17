
# Returns true if p1 wins and the number of turns taken
function playGame(p1::AbstractVector{Int8}, p2::AbstractVector{Int8})::Bool
    gamestates = Set{UInt}()
    # c = 0
    # take a turn
    while !isempty(p1) && !isempty(p2)

        # check for repeating gamestate
        gs = hash((p1, p2))
        if gs in gamestates return true#, c
        else push!(gamestates, gs) end

        # recurse into next game if both players have enough cards
        if p1[1]+1 <= length(p1) && p2[1]+1 <= length(p2)
            w = maximum(p1[2:p1[1]+1]) > maximum(p2[2:p2[1]+1]) ? true : playGame(p1[2:p1[1]+1], p2[2:p2[1]+1])
            # h = playGame(p1[2:p1[1]+1], p2[2:p2[1]+1])
            # w = h[1]
            # c += h[2]

        # determine round winner normally
        else w = p1[1] > p2[1] end

        # move cards to bottom of winners deck
        if w push!(p1, popfirst!(p1), popfirst!(p2))
        else push!(p2, popfirst!(p2), popfirst!(p1)) end

        # increment turn counter
        # c += 1
    end
    return length(p1) > length(p2)#, c
end

inpt = split.(split(read("AoC_20\\day22_input.txt", String), "\r\n\r\n"), "\r\n")

p1 = parse.(Int8, inpt[1][2:end])
p2 = parse.(Int8, inpt[2][2:end-1])

@time g = playGame(p1, p2)
println(sum(i*x for (i, x) in enumerate(reverse(g ? p1 : p2))))
