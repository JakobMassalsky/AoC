inpt = readlines("aoc_2020\\day13_input.txt")
times = parse.(Int, split(inpt[2], r"(,x)*,"))
remainders = abs.((parse(Int, inpt[1]) .% times)- times)
println(minimum(remainders) * times[findfirst(x->x==minimum(remainders), remainders)])
# function getfirst(c, i)
#     x = 0
#     while true
#         x += 1
#         test(c, x, i) && return x
#     end
# end
#
# function test(c, x, i)
#     return (c[1][2]*x+c[i][1])%c[i][2] == 0
# end

# function getPeriod(interval, offset, period, remainder)
#     for i in 0:period
#         (interval*i+offset+(remainder%period))%period == 0 && return [interval*period, i*interval+offset]
#     end
# end

inpt = split(readlines("aoc_2020\\day13_input.txt")[2], ",")
coll = Vector{Vector{Int}}()

for i in 1:length(inpt)
    inpt[i] != "x" && push!(coll, [i-1, parse(Int, inpt[i])])
end

function getPeriod(offInt, remPer)
    for i in 0:remPer[2]
        (offInt[2]*i+offInt[1]+(remPer[1]%remPer[2]))%remPer[2] == 0 && return [i*offInt[2]+offInt[1], offInt[2]*remPer[2]]
    end
end

println(reduce(getPeriod, coll)[1])
#
# tmp = [coll[1][2], coll[1][1]]
# for i in 1:length(coll)-1
#     global tmp = getPeriod(tmp[1], tmp[2], coll[i+1][2], coll[i+1][1])
# end


# println(getPeriod(coll[1][2], coll[1][1], coll[2][2], coll[2][1]))
# println(getPeriod(221, 102, coll[3][2], coll[3][1]))
# # occurances = []
#
# for i in 2:length(collection)
#     push!(occurances, getfirst(collection, i))
# end
#
# for i in 1:10000000
#     u = 6+13*i
#     u % 19 == 11 && (println(u*17); break)
# end
