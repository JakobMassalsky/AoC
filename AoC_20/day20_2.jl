getEdges(m::AbstractMatrix{Bool})::AbstractVector{AbstractVector{Bool}} = [getTopEdge(m), getRightEdge(m), getBottomEdge(m), getLeftEdge(m)]
getLeftEdge(m::AbstractMatrix{Bool})::AbstractVector{Bool} = [m[x, 1] for x in 1:10]
getRightEdge(m::AbstractMatrix{Bool})::AbstractVector{Bool} = [m[x, end] for x in 1:10]
getTopEdge(m::AbstractMatrix{Bool})::AbstractVector{Bool} = [m[1, x] for x in 1:10]
getBottomEdge(m::AbstractMatrix{Bool})::AbstractVector{Bool} = [m[end, x] for x in 1:10]
getInner(m::AbstractMatrix{Bool})::AbstractMatrix{Bool} = m[2:end-1, 2:end-1]

function compEdges(e::AbstractVector{Bool}, k::Int, dir::AbstractString)::Union{Nothing, AbstractMatrix{Bool}}
    for j in 1:4
        tiles[k] = rotr90(tiles[k])
        dir == "t" && getTopEdge(tiles[k]) == e && return pop!(tiles, k)
        dir == "l" && getLeftEdge(tiles[k]) == e && return pop!(tiles, k)
    end
end

function getNext(cu::AbstractMatrix{Bool}, dir::AbstractString)::AbstractMatrix{Bool}
    e = dir == "l" ? getRightEdge(cu) : getBottomEdge(cu)
    ne = nothing
    for k in keys(tiles)
        ne = ne == nothing ? compEdges(e, k, dir) : ne
        ne == nothing || break
        tiles[k] = tiles[k][end:-1:1,1:end]
        ne = compEdges(e, k, dir)
    end
    return ne
end

function findSM(im::AbstractMatrix{Bool}, sm::AbstractMatrix{Bool})::Int
    for i in 1:4
        im = rotr90(im)
        iP, iS = CartesianIndices(im), CartesianIndices(sm)
        o = oneunit(first(iP))
        c = sum(sum((im[i-o+j] == sm[j]) && sm[j] for j in iS) == 15 for i in CartesianIndex(1, 1):last(iP)-last(iS))
        c != 0 && return (sum(im)-c*15)
    end
end

# declare global variables
# inpt = split.(split(read("AoC_20\\day20_input_alex.txt", String), "\n\n"), "\n")
inpt = split.(split(read("AoC_20\\day20_input.txt", String), "\r\n\r\n"), "\r\n")
deleteat!(inpt[end], length(inpt[end]))
sz = Int(sqrt(length(inpt)))
pic, uPic = Matrix{Union{Nothing, Matrix{Bool}}}(nothing, sz, sz), Matrix{Union{Nothing, Bool}}(nothing, 8*sz, 8*sz)
sMstr = split.(split("                  # -#    ##    ##    ###- #  #  #  #  #  #   ", "-"), "")
sM = Matrix{Bool}(undef, 3, 20)
tiles = Dict([parse(Int, m[1][end-5:end-1]) => [m[i[2]+1][i[1]] == '#' for i in CartesianIndices(Matrix{Bool}(undef, 10, 10))] for m in inpt])

# find top left corner
for k in keys(tiles)
    ek = getEdges(tiles[k])
    matches = []
    # find edges of tiles[k] that match any other match of a tile in tiles
    for j in keys(tiles)
        if k != j
            ej = getEdges(tiles[j])
            for v in 1:length(ek)
                for w in 1:length(ej)
                    (ek[v] == ej[w] || ek[v] == reverse!(ej[w])) && push!(matches, v)
                end
            end
        end
    end
    # write tiels[k] to top left corner in correct orientation
    if length(sort!(matches)) == 2 && (pic[1, 1] == nothing)
        if matches == [1, 2] tiles[k] = rotr90(tiles[k])
        elseif matches == [3, 4] tiles[k] = rotl90(tiles[k])
        elseif matches == [1, 4] tiles[k] = rot180(tiles[k]) end
        pic[1, 1] = pop!(tiles, k)
        break
    end
end

# arrange tiles into global pic-Matrix
for y in 1:size(pic, 1)
    for x in 1:size(pic, 2)-1
        pic[y, x+1] = getNext(pic[y, x], "l")
    end
    y < size(pic, 1) && (pic[y+1, 1] = getNext(pic[y, 1], "t"))
end

# write contents of pic into 1 single matrix upic
for i in CartesianIndices(pic)
    tile = getInner(pic[i])
    for j in CartesianIndices(tile)
        uPic[(i-oneunit(i))*8+j] = tile[j]
    end
end

# make sea monster matrix
for i in CartesianIndices(sM)
    sM[i] = sMstr[i[1]][i[2]] == "#"
end

# count pixels that are not sea monsters and "#"
println([findSM(uPic, sM), findSM(uPic[end:-1:1,1:end], sM)])
println("Run, there are ", Int((sum(uPic)-[findSM(uPic, sM), findSM(uPic[end:-1:1,1:end], sM)][1]) / 15), " Sea Monsters on your Image!")
