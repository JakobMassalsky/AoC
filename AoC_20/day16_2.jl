inpt = split(read("AoC_20\\day16_input.txt", String), "\r\n\r\n")
rules = split.(split(inpt[1], "\r\n"), ": ")
myTicket = split(split(inpt[2], "\r\n")[2], ",")
otherTickets = split.(split(inpt[3], "\r\n")[2:end], ",")
pop!(otherTickets)
Values = Vector{Int}()

for s in rules
    r = parse.(Int, split(s[2], r"-|( or )"))
    append!(Values, append!(collect(r[1]:r[2]), collect(r[3]:r[4])))
end

j = 1
while j <= length(otherTickets)
    deleted = false
    for n in parse.(Int, otherTickets[j])
        n in Values || (deleteat!(otherTickets, j); deleted = true; break)
    end
    global j += deleted ? 0 : 1
end

ranges = []

for i in 1:length(rules)
    r = parse.(Int, split(rules[i][2], r"-|( or )"))
    push!(ranges, (rules[i][1], append!(collect(r[1]:r[2]), collect(r[3]:r[4]))))
end
# println(ranges)
ruleDistribution = []

for i in 1:20
    groups = []
    for t in otherTickets
        validRules = copy(ranges)
        val = parse(Int, t[i])
        names = Vector{String}()
        z = 1
        while z <= length(validRules)
            val in validRules[z][2] && push!(names, validRules[z][1])
            val in validRules[z][2] || (deleteat!(validRules, z);z -= 1)
            z += 1
        end
        push!(groups, names)
    end
    push!(ruleDistribution, (reduce(intersect, groups)))
end

while !all(length.(ruleDistribution) .== 1)
    for r in ruleDistribution
        if length(r) == 1
            for s in ruleDistribution
                while length(s) != 1 && findnext(s .== r[1], 1) != nothing
                    deleteat!(s, findnext(s .== r, 1))
                end
            end
        end
    end
end

acc = prod(collect([contains(ruleDistribution[i][1], "departure") ? parse(Int, myTicket[i]) : 1 for i in 1:length(ruleDistribution)]))
println(acc)
# for i in 1:length(ruleDistribution)
#     global acc *= contains(ruleDistribution[i][1], "departure") ? parse(Int, myTicket[i]) : 1
#     contains(ruleDistribution[i][1], "departure") && println(ruleDistribution[i])
# end
