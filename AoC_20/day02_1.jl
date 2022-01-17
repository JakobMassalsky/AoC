inpt = readlines("AoC_20\\day02_input.txt")
validCounter1 = 0
validCounter2 = 0

for i in 1:length(inpt)

    low = inpt[i][2] == "-"[1] ? parse(Int, inpt[i][1]) : (parse(Int, inpt[i][1:2]))
    h = Int(floor(log10(low)))
    high = inpt[i][4+h] == " "[1] ? parse(Int, inpt[i][3+h]) : (parse(Int, inpt[i][3+h:4+h]))

    c = "."[1]
    beg = 0
    for j in 1:length(inpt[i])
        inpt[i][j] == ":"[1] && (c = inpt[i][j-1]; beg = j+1)
    end

    sum(help = inptline1 .== "c")

    count = -1
    for j in 1:length(inpt[i])
        count += inpt[i][j] == c ? 1 : 0
    end

    global validCounter2 += xor(inpt[i][beg+low] == c, inpt[i][beg+high] == c) ? 1 : 0

    global validCounter1 += count >= low && count <= high ? 1 : 0

end
