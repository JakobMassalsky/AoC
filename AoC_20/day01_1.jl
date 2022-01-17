parsed = parse.(Int,readlines("AoC_20\\day01_input.txt"))
solution = 0

lower = []
upper = []

for i in parsed
    i < 1010 && push!(lower, i)
    i > 1010 && push!(upper, i)
end

for i in lower
    for n in upper
        (i + n == 2020) && (global solution = i*n)
    end
end

println(solution)

for i in lower
    for n in lower
        for l in parsed
            (i + n + l == 2020) && (global solution = i*n*l)
        end
    end
end

println(solution)
