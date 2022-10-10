include("intcodeComputer.jl")

programm = "3,8,1001,8,10,8,105,1,0,0,21,42,67,84,97,118,199,280,361,442,99999,3,9,101,4,9,9,102,5,9,9,101,2,9,9,1002,9,2,9,4,9,99,3,9,101,5,9,9,102,5,9,9,1001,9,5,9,102,3,9,9,1001,9,2,9,4,9,99,3,9,1001,9,5,9,1002,9,2,9,1001,9,5,9,4,9,99,3,9,1001,9,5,9,1002,9,3,9,4,9,99,3,9,102,4,9,9,101,4,9,9,102,2,9,9,101,3,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99"

#programm = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"

mutable struct Amplifier
    phase
    input
    pauseState
    pausePointer
end

possiblePhases = []

for i in 56789:98765
    criteria = 0
    for j = 1:5
        criteria += getDigit(i, j) < 5 ? 1 : 0
        for n = 1:5
            criteria += j != n ? getDigit(i, j) == getDigit(i, n) ? 1 : 0 : 0
        end
    end
    criteria == 0 && push!(possiblePhases, i)
end

prog = convertInput(programm)
AmplifierSets = []

for i = possiblePhases
    for j = 1:5
        push!(AmplifierSets, Amplifier(getDigit(i, j), 0, prog, 1))
    end
end

out = 0
outputs = []

for k = 0:119
    out = 0
    states = [0, 1, prog]
    while AmplifierSets[5k + 5].pausePointer != Inf
        for j = 1:5
            states = Run(AmplifierSets[5k + j].pauseState, AmplifierSets[5k + j].phase, AmplifierSets[5k + j].input, AmplifierSets[5k + j].pausePointer)
            AmplifierSets[5k + (j % 5) + 1].input = states[1]
            AmplifierSets[5k + j].pausePointer = states[2]
            AmplifierSets[5k + j].pauseState = states[3]
        end
    end
    # print("Output is: ")
    push!(outputs, states[1])
    # println("Cycle completed")
end

high = -Inf
for i = outputs
    global high = i > high ? i : high
end
print(high)
