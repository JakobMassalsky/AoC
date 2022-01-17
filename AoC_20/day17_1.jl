inpt = readlines("AoC_20\\day17_input.txt")
len = length(inpt)
dim = Dict{Tuple{Int, Int, Int}, Bool}()
steps = 6

for y in 1:2*steps+len
    for x in 1:2*steps+len
        for z in 1:2*steps+1
            (z == 7 && y in 1+steps:steps+len && x in 1+steps:steps+len) && push!(dim, (x, y, z) => inpt[y-steps][x-steps] == '#' ? true : false)
            !(z == 7 && y in 1+steps:steps+len && x in 1+steps:steps+len) && push!(dim, (x, y, z) => false)
        end
    end
end

function getNearby(dict, k)
    nearbyActive = 0
    for x in -1:1
        for y in -1:1
            for z in -1:1
                nk = (k[1]+x, k[2]+y, k[3]+z)
                nearbyActive += haskey(dim, nk) && dim[nk] ? 1 : 0
            end
        end
    end
    return nearbyActive
end

for i in 1:steps
    dmHelp = copy(dim)
    for k in keys(dim)
        nearbyActive = getNearby(dim, k)
        dmHelp[k] = !dim[k] ? nearbyActive == 3 ? true : false : nearbyActive >= 3 && nearbyActive <= 4 ? true : false
    end
    global dim = dmHelp
end

value = sum([x ? 1 : 0 for x in values(dim)])
