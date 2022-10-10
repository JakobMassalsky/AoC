inpt = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,9,23,1,5,23,27,1,27,9,31,1,6,31,35,2,35,9,39,1,39,6,43,2,9,43,47,1,47,6,51,2,51,9,55,1,5,55,59,2,59,6,63,1,9,63,67,1,67,10,71,1,71,13,75,2,13,75,79,1,6,79,83,2,9,83,87,1,87,6,91,2,10,91,95,2,13,95,99,1,9,99,103,1,5,103,107,2,9,107,111,1,111,5,115,1,115,5,119,1,10,119,123,1,13,123,127,1,2,127,131,1,131,13,0,99,2,14,0,0"
tmp = split(inpt, ",")
intcodeArray = []
for i = 1:length(tmp)
    push!(intcodeArray, parse(Int, tmp[i]))
end

value = 19690720

function Run(code)
    local intcode = copy(code)
    for i = 0:(div(length(intcode), 4) -1)
        i = 4i + 1
        if intcode[i] == 1
            intcode[intcode[i+3] + 1] = intcode[intcode[i+1] + 1] + intcode[intcode[i+2] + 1]
        elseif intcode[i] == 2
            intcode[intcode[i+3] + 1] = intcode[intcode[i+1] + 1] * intcode[intcode[i+2] + 1]
        end
        intcode[i] == 99 && break
    end
    return intcode
end

for i in 0:99
    for j in 0:99
        local inAr = copy(intcodeArray)
        inAr[2] = i
        inAr[3] = j
        (Run(inAr)[1]) == value && println(100i + j)
    end
end
