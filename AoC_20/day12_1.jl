inpt = readlines("aoc_2020\\day12_input.txt")

function degToVec(deg)
    return [floor(Int, sind(deg)), floor(Int, cosd(deg))]
end

Dirs = Dict('N'=>[1,0], 'S'=>[-1,0], 'E'=>[0,1], 'W'=>[0,-1])

pos = [0, 0]
dir = 0

for i in inpt
    val = parse(Int, i[2:end])
    global pos += contains(i, r"[NESW]") ? Dirs[i[1]].*val : i[1] == 'F' ? degToVec(dir).*val : [0,0]
    global dir += i[1] == 'L' ? val : i[1] == 'R' ? -val : 0
end
println(sum(abs.(pos)))
