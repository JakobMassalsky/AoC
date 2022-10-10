inpt = split.(split(read("aoc_2020\\day06_input.txt", String), "\r\n\r\n"), "\r\n")
counter = 0
alph = collect('a':'z')
# for g in inpt
#     global counter += length(intersect(collect.(g)))
# end

for g in inpt
    global counter += length(intersect(alph, intersect(collect.(g))))
end

# println(temp)
    # for c in alph
    #     occ = true
    #     for s in g
    #         occ = contains(s, c) && occ
    #     end
    #     occ && global counter += 1
    # end
