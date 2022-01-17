inpt = readlines("AoC_20\\day08_input.txt")

ip = 1

function runCode(ip, program)
    acc = 0
    while ip <= length(program) && program[ip] != ""
        if contains(program[ip], "acc")
            acc += parse(Int, program[ip][5:end])
        elseif contains(program[ip], "jmp")
            tmp = program[ip]
            program[ip] = ""
            ip += parse(Int, tmp[5:end])
            continue
        end
        program[ip] = ""
        ip += 1
    end
    return (acc, ip > length(program))
end

tuples = []

for i in 1:length(inpt)
    if contains(inpt[i], "jmp")
        inpt[i] = replace(inpt[i], "jmp" => "nop")
        push!(tuples, runCode(1, copy(inpt)))
        inpt[i] = replace(inpt[i], "nop" => "jmp")
    elseif contains(inpt[i], "jmp")
        inpt[i] = replace(inpt[i], "nop" => "jmp")
        push!(tuples, runCode(1, copy(inpt)))
        inpt[i] = replace(inpt[i], "jmp" => "nop")
    end
end

for t in tuples
    t[2] && println(t)
end
