function eval(d::String)

    n = 0
    s, e = Vector{Int}(), Vector{Int}()
    for i in 1:length(d)
        n += d[i] == '(' ? 1 : d[i] == ')' ? -1 : 0
        d[i] == '(' && n == 1 && (pushfirst!(s, i))
        d[i] == ')' && n == 0 && (pushfirst!(e, i))
    end

    for i in 1:length(s)
        d = d[1:(s[i]-1)] * string(eval(d[(s[i]+1):(e[i]-1)])) * d[(e[i]+1):end]
    end
    
    d = split(d, " ")
    acc = parse(Int, d[1])
    pr = 1
    for i in 2:2:length(d)-1
        d[i] == "*" && (pr *= acc; acc = 0)
        acc += parse(Int, d[i+1])
    end

    return pr*acc::Int
end

println(sum([eval(l) for l in readlines("AoC_20\\day18_input.txt")]))
