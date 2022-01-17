cups = [1,6,7,2,4,8,3,5,9]
# cups = [3,8,9,1,2,5,4,6,7]
s = 9
for i in 1:100
    pc = splice!(cups, 2:4)
    first = cups[1]
    l = (first+7)%9+1
    while l in pc
        l = (l+7)%9+1
    end
    l = findfirst(cups .== l)
    global cups = append!(cups[2:l], append!(pc, cups[l+1:end]))
    push!(cups, first)
end

# @time for i in 1:100
#     off, p = s-2, po+1
#     pc = splice!(cups, p:p+2)
#     l = (cups[p]+off)%s+1
#     while l in pc
#         l = (l+off)%s+1
#     end
#     # println(l)
#     l = findfirst(isequal(l), cups)
#     for x in 1:3
#         insert!(cups, l+x, pc[x])
#     end
#     global po = (po+1)%s
#     # push!(cups, popfirst!(cups))
# end

println(cups)
