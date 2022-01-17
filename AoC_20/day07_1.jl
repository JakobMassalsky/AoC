inpt = readlines("AoC_20\\day07_input3.txt")
inpt = split.(replace.(replace.(replace.(inpt, r"bags|bag" => ""), r" , " => ","), r" \." => ""), "  contain ")

mutable struct bag
    color::String
    contents::Vector{String}
    containedIn::Vector{String}
end

function returnContains(k::String)
    tmp = []
    for s in bagDict[k].contents
        s != "no other" && (push!(tmp, (s)); append!(tmp, returnContains(replace(s, r"\d " => ""))))
    end
    return tmp
end

function getCount(n::Int, k::String)
    tmp = n
    for s in bagDict[k].contents
        d = (s != "no other") ? parse(Int, split(s, " ")[1]) : undef
        tmp += s != "no other" ? getCount(d*n, replace(s, r"\d " => "")) : 0
    end
    return tmp
end

bagDict = Dict{String, bag}()
counter = 0

for b in inpt
    push!(bagDict, b[1] => bag(b[1], split(b[2], ","), []))
end

for k in keys(bagDict)
    global counter += contains(join(unique(returnContains(k))), "shiny gold") ? 1 : 0
end

cnt = getCount(1, "shiny gold")
println(cnt)
