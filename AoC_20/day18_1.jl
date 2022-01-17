function eval1(d::String)

    n = 0
    s, e = Vector{Int}(), Vector{Int}()
    for i in 1:length(d)
        n += d[i] == '(' ? 1 : d[i] == ')' ? -1 : 0
        d[i] == '(' && n == 1 && (pushfirst!(s, i))
        d[i] == ')' && n == 0 && (pushfirst!(e, i))
    end

    for i in 1:length(s)
        d = d[1:(s[i]-1)] * string(eval1(d[(s[i]+1):(e[i]-1)])) * d[(e[i]+1):end]
    end

    d = split(d, " ")
    acc = parse(Int, d[1])
    for i in 2:2:length(d)-1
        acc += d[i] == "+" ? parse(Int, d[i+1]) : acc*(parse(Int, d[i+1])-1)
    end

    return acc::Int
end

function eval2(d::Array{SubString{String},1})
    op, acc, i = "+", 0, 1

    while i in 1:length(d)
        if d[i] == "("
            arr = eval2(d[i+1:end])
            acc += op == "+" ? arr[1] : acc*(arr[1]-1)
            i += arr[2]
            continue end
        d[i] == ")" && return [acc, i+1]
        acc += contains(d[i], r"\d") ? op == "+" ? parse(Int, d[i]) : acc*(parse(Int, d[i])-1) : 0
        op = contains(d[i], r"[\*\+]") ? d[i] : op
        i += 1
    end

    return [acc, i+1]
end

println(sum([eval2(split(replace(l, " " => ""), ""))[1] for l in readlines("AoC_20\\day18_input.txt")]))
