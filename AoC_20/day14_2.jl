inpt = readlines("AoC_20\\day14_input.txt")
# mask = []
# memory = Dict{Int, Int}()
# acc = 0
function fillDict(inpt)
    mask = []
    memory = Dict{Int, Int}()
    for s in inpt
        if contains(s, "ma")
            mask = collect(split(s, " = ")[2])
        end
        if contains(s, "me")
            val = parse(Int, split(s, " = ")[2])
            add = collect(string(parse(Int, split(s, r"[\[\]]")[2]), base=2))
            while length(add) < length(mask)
                pushfirst!(add, '0')
            end
            result = ""
            for i in 1:length(mask)
                result *= mask[i] == '0' ? add[i] : mask[i]
            end

            for i in 0:(2^length(collect(findall(collect(result) .== 'X'))))-1
                cast = collect(string(i, base=2))

                while length(cast) < length(collect(findall(collect(result) .== 'X')))
                    pushfirst!(cast, '0')
                end

                cnt = 0
                tmp = []

                for c in collect(result)
                    cnt += c == 'X' ? 1 : 0
                    push!(tmp, c == 'X' ? cast[cnt] : c)
                end

                memory[parse(Int, join(tmp), base=2)] = val

            end

            # global acc += 2^length(collect(findall(collect(result) .== 'X')))

            # global memory[] = parse(Int, result, base=2)
        end
    end
    return sum(values(memory))
end

@time fillDict(inpt)

println(fillDict(inpt))
#
# erg
# println(erg)
