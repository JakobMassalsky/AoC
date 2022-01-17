inpt = collect.("." .* readlines("AoC_20\\day11_input.txt") .* ".")
dirs = [[-1, -1], [0, -1], [1, -1], [1, 0], [1, 1], [-1, 0], [-1, 1] ,[0, 1]]

for i in 1:1000
    tmp = copy.(inpt)
    for y in 2:(length(inpt)-1)
        for x in 2:(length(inpt[y])-1)
            # inpt[y][x] == '.' && continue
            if inpt[y][x] == 'L'
                valid = true
                for d in dirs
                    p = [y,x] + d
                    while p[1] >= 1 && p[2] >= 1 && p[1] <= length(inpt) && p[2] <= length(inpt[y])
                        valid = valid && inpt[p[1]][p[2]] != '#'
                        inpt[p[1]][p[2]] != '.' && break
                        p += d
                    end
                end
                tmp[y][x] = valid ? '#' : 'L'
            elseif inpt[y][x] == '#'
                c = 0
                for d in dirs
                    p = [y,x] + d
                    while p[1] >= 1 && p[2] >= 1 && p[1] <= length(inpt) && p[2] <= length(inpt[y])
                        c += inpt[p[1]][p[2]] == '#' ? 1 : 0
                        inpt[p[1]][p[2]] != '.' && break
                        p += d
                    end
                end
                tmp[y][x] = c >= 5 ? 'L' : '#'
            end
        end
    end
    inpt == tmp && break
    global inpt = copy.(tmp)
end
counter = 0
for s in inpt
    for c in s
        global counter += c == '#' ? 1 : 0
    end
end
