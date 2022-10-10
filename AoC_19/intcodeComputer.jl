function convertInput(input)
    tmp = split(input, ",")
    intcodeArray = []
    for i = 1:length(tmp)
        push!(intcodeArray, parse(Int, tmp[i]))
    end
    return intcodeArray
end

function getDigit(n, i)
    return i > 0 ? (n % 10^i - n % 10^(i-1)) / 10^(i-1) : 0
end

function Run(intcode, phase, input, startAt, relativeB)
    global i = startAt
    global relativeBase = relativeB
    output = 0
    # println("Initializing Intcode Computer")
    laenge = length(intcode)
    for i = 1:100
        push!(intcode, 0)
    end

    while i <= laenge
        mode1 = getDigit(intcode[i], 3)
        mode2 = getDigit(intcode[i], 4)
        mode3 = getDigit(intcode[i], 5)
        opCode = intcode[i] % 100

        # println("Operation ", opCode," called (", intcode[i], "), first Parameter called: ", intcode[i+1], ", second Parameter called: ", intcode[i+2])
        # println("Modes called: Mode1 = ", mode1, ", Mode2 = ", mode2)
        if opCode == 1
            intcode[intcode[i+3] + 1 + (mode3 == 2 ? relativeBase : 0)] = operation1(intcode, mode1, mode2, i, relativeBase)
            i += 4
        elseif opCode == 2
            intcode[intcode[i+3] + 1 + (mode3 == 2 ? relativeBase : 0)] = operation2(intcode, mode1, mode2, i, relativeBase)
            i += 4
        elseif opCode == 3
            intcode[intcode[i+1] + 1 + (mode1 == 2 ? relativeBase : 0)] = i == 1 ? phase : input
            i += 2
            # For BlockBreaker
            return [Inf, i, intcode, false, relativeBase]
        elseif opCode == 4
            output = operation4(intcode, mode1, i, relativeBase)
            return intcode[i+2] == 99 ? [output, 1, intcode, true, relativeBase] : [output, i+2, intcode, false, relativeBase]
        elseif opCode == 5
            i = operation5(intcode, mode1, mode2, i, relativeBase)
        elseif opCode == 6
            i = operation6(intcode, mode1, mode2, i, relativeBase)
        elseif opCode == 7
            intcode[intcode[i+3] + 1 + (mode3 == 2 ? relativeBase : 0)] = operation7(intcode, mode1, mode2, i, relativeBase)
            i += 4
        elseif opCode == 8
            intcode[intcode[i+3] + 1 + (mode3 == 2 ? relativeBase : 0)] = operation8(intcode, mode1, mode2, i, relativeBase)
            i += 4
        elseif opCode == 9
            relativeBase += operation9(intcode, mode1, i, relativeBase)
            # println("Relative Base adjusted!")
            i += 2
        end
        # println("Int 494 = ", intcode[494])
        opCode == 99 && break
        # println("cycle completed")
    end
    return [output, 1, intcode, true, relativeBase]
end

function evaluate(intcode, mode, i, rB)
    return convert(Int, mode == 0 ? intcode[convert(Int, intcode[i]+1)] : mode == 2 ? intcode[convert(Int, intcode[i]+1 + relativeBase)] : intcode[i])
end

function operation1(intcode, mode1, mode2, i, rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 + parameter2
end

function operation2(intcode, mode1, mode2, i, rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 * parameter2
end

function operation4(intcode, mode1, i, rB)
    return evaluate(intcode, mode1, i+1, rB)
end

function operation5(intcode, mode1, mode2, i, rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 != 0 ? parameter2 + 1 : i + 3
end

function operation6(intcode, mode1, mode2, i, rB)
    # println("If ", intcode[i+1], " = 0, set Index to evaluated ", intcode[i+2], " Relative Base: ", rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 == 0 ? parameter2 + 1 : i + 3
end

function operation7(intcode, mode1, mode2, i, rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 < parameter2 ? 1 : 0
end

function operation8(intcode, mode1, mode2, i, rB)
    parameter1 = evaluate(intcode, mode1, i+1, rB)
    parameter2 = evaluate(intcode, mode2, i+2, rB)
    return parameter1 == parameter2 ? 1 : 0
end

function operation9(intcode, mode, i, rB)
    return evaluate(intcode, mode, i+1, rB)
end
