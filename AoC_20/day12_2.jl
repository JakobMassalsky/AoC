inpt = readlines("aoc_2020\\day12_input.txt")

function rotateVecLeft(vec, deg)
    deg == 180 && return vec.*-1
    return [floor(Int, sind(deg))*vec[2], -floor(Int, sind(deg))*vec[1]]
end

Dirs = Dict('N'=>[1,0], 'S'=>[-1,0], 'E'=>[0,1], 'W'=>[0,-1])

pos = [0, 0]
waypos = [1, 10]

for i in inpt
    val = parse(Int, i[2:end])
    global waypos += contains(i, r"[NESW]") ? Dirs[i[1]].*val : [0,0]
    global waypos = i[1] == 'L' ? rotateVecLeft(waypos, val) : i[1] == 'R' ? rotateVecLeft(waypos, 360-val) : waypos
    global pos += i[1] == 'F' ? waypos.*val : [0,0]
end
println(sum(abs.(pos)))
