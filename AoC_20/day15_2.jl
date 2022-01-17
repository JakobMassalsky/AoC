using DataStructures

spokenNumbers = Dict{Int32, Int32}(0 => 1,13 => 2,1 => 3,16 => 4,6 => 5)
offset = length(spokenNumbers)
lastNumber = 17
turnCount = 30000000

function getNumber(spokenNumbers, offset, lastNumber, turnCount)
    for i in 1:turnCount-offset-1
        if haskey(spokenNumbers, lastNumber)
            spokenNumber = i + offset - spokenNumbers[lastNumber]
            spokenNumbers[lastNumber] = i + offset
        else
            push!(spokenNumbers, lastNumber=>i + offset)
            spokenNumber = 0
        end
        lastNumber = spokenNumber
    end
    return lastNumber
end

@time getNumber(spokenNumbers, offset, lastNumber, turnCount)

# println
# spokenNumbers = SortedDict{Int32, Int32}(0 => 1,13 => 2,1 => 3,16 => 4,6 => 5)
#
# @time getNumber(spokenNumbers, offset, lastNumber, turnCount)
