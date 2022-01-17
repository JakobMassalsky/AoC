spokenNumbers = [0,13,1,16,6]
offset = length(spokenNumbers)
lastNumber = 17
turnCount = 2020

for i in 1:turnCount-offset-1
    spokenNumber = lastNumber in spokenNumbers ? i + offset - findlast(spokenNumbers .== lastNumber) : 0
    push!(spokenNumbers, lastNumber)
    global lastNumber = spokenNumber
end
