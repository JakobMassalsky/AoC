inpt = readlines("AoC_20\\day03_input.txt")
counter = [0, 0, 0, 0, 0]

for y in 2:length(inpt)
    tree1 = inpt[y][((y-1))%31+1] == "#"[1]
    tree2 = inpt[y][((y-1)*3)%31+1] == "#"[1]
    tree3 = inpt[y][((y-1)*5)%31+1] == "#"[1]
    tree4 = inpt[y][((y-1)*7)%31+1] == "#"[1]
    tree5 = isodd(y) && inpt[y][Int((y-1)/2)%31+1] == "#"[1]
    global counter += [tree1 ? 1 : 0, tree2 ? 1 : 0, tree3 ? 1 : 0, tree4 ? 1 : 0, tree5 ? 1 : 0, ]
end
println(prod(counter))
