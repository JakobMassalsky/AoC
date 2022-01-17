inpt = readlines("AoC_20\\day14_input.txt")
mask = []
memory = Dict{Int, Int}()

for s in inpt
    if contains(s, "ma")
        global mask = collect(split(s, " = ")[2])
    end
    if contains(s, "me")
        val = collect(string(parse(Int, split(s, " = ")[2]), base=2))
        while length(val) < length(mask)
            pushfirst!(val, '0')
        end
        result = ""
        # println(val)
        for i in 1:length(mask)
            result *= mask[i] == 'X' ? val[i] : mask[i]
        end
        global memory[parse(Int, split(s, r"[\[\]]")[2])] = parse(Int, result, base=2)
    end
end

println(sum(values(memory)))
