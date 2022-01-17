inpt = split.(split(read("AoC_20\\day19_input.txt", String), "\r\n\r\n"), "\r\n")
rules = Dict(r[1] => replace(contains(r[2], "|") ? "( " * r[2] * " )" : r[2], "\"" => "") for r in split.(inpt[1], ": "))

f = x -> contains(x, r"\d") ? f(join([contains(s, r"\d") ? rules[s] : s for s in split(x, r"( )")], " ")) : x
counter = sum(contains.(inpt[2][1:end-1], Regex("^" * replace(f("0"), r" " => "") * "\$")))


# mkReg(reg::AbstractString) = contains(reg, r"\d") ? mkReg(join([contains(s, r"\d") ? rules[s] : s for s in split(reg, r"( )")], " ")) : reg
# reg = mkReg("0")

# while contains(reg, r"\d")
#     global reg = join([contains(s, r"\d") ? rules[s] : s for s in split(reg, r"( )")], " ")
# end

# counter = sum(map(x -> contains(x, Regex("^" * replace(f("0"), r" " => "") * "\$")), inpt[2][1:end-1]))

# while contains(reg, r"\d")
#     global reg = join(map(x -> contains(x, r"\d") ? rules[x] : x, split(reg, r"( )")), " ")
# end
