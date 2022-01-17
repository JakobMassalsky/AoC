inpt = split.(split(read("AoC_20\\day19_input.txt", String), "\r\n\r\n"), "\r\n")
rules = Dict(r[1] => replace(contains(r[2], "|") ? "( " * r[2] * " )" : r[2], "\"" => "") for r in split.(inpt[1], ": "))

f = x -> contains(x, r"\d") ? f(join([contains(s, r"\d") ? s == "8" ? "( 42 ) +" : s == "11" ? "( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )" : rules[s] : s for s in split(x, r"( )")], " ")) : x
counter = sum(contains.(inpt[2][1:end-1], Regex("^" * replace(f("0"), r" " => "") * "\$")))


# function mkReg(reg::AbstractString)
#     return contains(reg, r"\d") ? mkReg(join([contains(s, r"\d") ? s == "8" ? "( 42 ) +" : s == "11" ? "( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )" : rules[s] : s for s in split(reg, r"( )")], " ")) : reg
# end
# counter = sum(contains.(inpt[2][1:end-1], Regex("^" * replace(mkReg("0"), r" " => "") * "\$")))


# while contains(reg, r"\d")
#     global reg = join([contains(s, r"\d") ? s == "8" ? "( 42 ) +" : s == "11" ? "( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )" : rules[s] : s for s in split(reg, r"( )")], " ")
# end


# println("Recoursive: ")
# @time mkReg("0")
# println("Iterative: ")
# @time mkReg2("0")
# @time sum(contains(m, Regex("^" * replace(mkReg("0"), r" " => "") * "\$")) for m in msg)

# function mkReg2(reg::AbstractString)
#     while contains(reg, r"\d")
#         reg = join([contains(s, r"\d") ? s == "8" ? "( 42 ) +" : s == "11" ? "( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )" : rules[s] : s for s in split(reg, r"( )")], " ")
#     end
#     return reg
# end
# tmp = ""
# for s in split(reg, r"( )+")
    # if contains(s, r"\d")
# tmp *= contains(s, r"\d") ? s == "8" ? " ( 42 ) +" : s == "11" ? " ( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )" : " "*rules[s] : " "*s
    #     if s == "8"
    #         tmp *= " ( 42 ) +"
    #     elseif s == "11"
    #         tmp *= " ( 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 )"
    #     end
    # else tmp *= " "*s end
# end
