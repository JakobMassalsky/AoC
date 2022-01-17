inpt = collect.("." .* readlines("AoC_20\\day11_input.txt") .* ".")

for i in 1:10000
    tmp = copy.(inpt)
    for y in 2:(length(inpt)-1)
        for x in 2:(length(inpt[y])-1)
            # inpt[y][x] == '.' && continue
            if inpt[y][x] == 'L'
                valid = true
                for xt in x-1:x+1
                    for yt in y-1:y+1
                        valid = valid && inpt[yt][xt] != '#'
                    end
                end
                tmp[y][x] = valid ? '#' : 'L'
            elseif inpt[y][x] == '#'
                c = 0
                for xt in x-1:x+1
                    for yt in y-1:y+1
                        c += inpt[yt][xt] == '#' ? 1 : 0
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
