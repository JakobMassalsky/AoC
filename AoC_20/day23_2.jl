cups = [1,6,7,2,4,8,3,5,9]
# cups = [3,8,9,1,2,5,4,6,7]
s, c = 1000000, 10000000
ca = append!(cups, collect(10:s))
cl = Vector{Int}(undef, s)
for i in 1:length(ca)
    cl[ca[i]] = ca[i==s ? 1 : i+1]
end

function shuffle(cups::Vector{Int}, p::Int, cycles::Int)
    s = length(cups)
    for i in 1:cycles
        off, next1 = s-2, cups[p]
        next2 = cups[next1]
        next3 = cups[next2]
        val = (p+off)%s+1
        while val == next1 || val == next2 || val == next3
            val = (val+off)%s+1
        end
        cups[p] = cups[next3]
        cups[next3] = cups[val]
        cups[val] = next1
        p = cups[p]
    end
    cups
end

@time shuffle(cl, 1, c)
println(cl[1]*cl[cl[1]])
