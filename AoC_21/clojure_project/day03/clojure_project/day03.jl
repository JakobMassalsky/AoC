inpt = readlines("C:/Users/jakob/OneDrive/Desktop/Coding/AoC_21/clojure_project/day03/clojure_project/input.txt")

inpt = [parse.(Int, x) for x in split.(inpt, "")]
res = reduce(+, inpt)
gamma = parse(Int, join([x > length(inpt)/2 ? "1" : "0" for x in res]), base=2)

println(gamma * xor(gamma, 0b111111111111))

inpt2 = inpt

for i in 1:12
    res[i] = reduce(+, inpt)[i] >= length(inpt)/2
    global inpt = [x for x in inpt if x[i] == res[i]]
    if length(inpt) <= 1
        break
    end
end

o2 = parse(Int, join([x == 1 ? "1" : "0" for x in inpt[1]]), base=2)

for i in 1:12
    res[i] = reduce(+, inpt2)[i] >= length(inpt2)/2
    global inpt2 = [x for x in inpt2 if x[i] != res[i]]
    if length(inpt2) <= 1
        break
    end
end

co2 = parse(Int, join([x == 1 ? "1" : "0" for x in inpt2[1]]), base=2)
println(co2 * o2)
