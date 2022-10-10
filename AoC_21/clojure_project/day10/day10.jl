inpt = readlines("aoc2021\\input.txt")
parpairs = Dict{Char, Char}('('=>')', '['=>']', '{'=>'}', '<'=>'>')
parpoints = Dict{Char, Int}(')'=>3, ']'=>57, '}'=>1197, '>'=>25137, '('=>1, '['=>2, '{'=>3, '<'=>4)
errscore = 0
incomplete = []

for line in inpt
    ppVector = []
    for par in line
        if par in keys(parpairs)
            push!(ppVector, par)
        elseif parpairs[ppVector[end]] == par
            pop!(ppVector)
        else
            global errscore += parpoints[par]
            ppVector = []
            break
        end
    end # for
    ppVector == [] && continue;
    push!(incomplete, reduce((x, y) -> 5*x + y, parpoints[x] for x in reverse(ppVector)))
end

println(errscore)
println(sort(incomplete)[Int(ceil(length(incomplete)/2))])
