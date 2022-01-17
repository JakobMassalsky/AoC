using DataStructures

inpt = split.(readlines("AoC_20\\day21_input.txt"), " (contains ")
ing = split.([x[1] for x in inpt], " ")
ag = split.(replace.([x[2] for x in inpt], ")" => ""), ", ")
al = DataStructures.SortedDict{AbstractString, Array{AbstractString}}()

for i in 1:length(ag)
    for a in ag[i]
        if haskey(al, a) al[a] = intersect(al[a], ing[i])
        else push!(al, a => ing[i]) end
    end
end

defAll = reduce(union, values(al))
c = sum([i in defAll ? 0 : 1 for f in ing for i in f])

while !all(length.(values(al)) .== 1)
    for k in keys(al)
        if length(al[k]) == 1
            for j in keys(al)
                if k != j && al[k][1] in al[j]
                    popat!(al[j], findfirst(al[j] .== al[k][1]))
                end
            end
        end
    end
end

println(join([al[k][1] for k in keys(al)], ","))
