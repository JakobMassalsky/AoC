inpt = split.(split(read("AoC_20\\day22_input.txt", String), "\r\n\r\n"), "\r\n")


p1 = parse.(Int, inpt[1][2:end])
p2 = parse.(Int, inpt[2][2:end-1])
c = 0
@time while !isempty(p1) && !isempty(p2)
    global c += 1
    if p1[1] > p2[1] append!(p1, [popfirst!(p1), popfirst!(p2)])
    else append!(p2, [popfirst!(p2), popfirst!(p1)]) end
end

println(sum(i*x for (i, x) in enumerate(reverse(isempty(p2) ? p1 : p2))))
