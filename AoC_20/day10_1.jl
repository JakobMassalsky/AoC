inpt = sort(parse.(Int, readlines("AoC_20\\day10_input.txt")))
pushfirst!(inpt, 0)
sets = Vector{Vector{Int}}()
last = 0
singleSteps = 0
tripleSteps = 0
lastTriple = 1

function numArrangements(v::Vector{Int})
    if length(v) <= 2 return 1
    elseif length(v)-2 > 2
        return (2^(length(v)-2))-1
    else return 2^(length(v)-2) end

end

for i in 1:length(inpt)
    global singleSteps += inpt[i]-last == 1 ? 1 : 0
    global tripleSteps += inpt[i]-last == 3 ? 1 : 0
    inpt[i]-last == 3 && (push!(sets, inpt[lastTriple:i-1]); global lastTriple = i)
    global last = inpt[i]
end
push!(sets, inpt[lastTriple:end])

println(singleSteps*(tripleSteps+1))
acc = 1
for s in sets
    global acc *= numArrangements(s)
end
println(acc)
