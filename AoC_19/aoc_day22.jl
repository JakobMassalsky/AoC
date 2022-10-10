function dealNewStack(stack::Vector{Int64})
    return reverse(stack)
end

function cut(cards::Vector{Int64}, i::Int64)
    tmp = view(cards, i > 0 ? (1:i) : (i:length(cards)))
    return i > 0 ? append!(view(cards, i:length(cards)), tmp) : append!(tmp, view(cards, (1:i)))
end

function dealWith()
    dd
end

cards = Vector{Int64}(undef, 0)

for i in 0:10006
    push!(cards, i)
end

cards = dealNewStack(cards)

cards = cut(cards, 10)
print(cards)
