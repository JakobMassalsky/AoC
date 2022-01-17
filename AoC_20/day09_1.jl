inpt = parse.(Int, readlines("AoC_20\\day09_input.txt"))
sol = 0
index = 0

for i = 26:length(inpt)
    valid = false
    for k in i-25:i-1
        if !valid
            for l in i-25:i-1
                valid = inpt[k]+inpt[l]==inpt[i] || valid
            end
        end
    end
    !valid && (println(inpt[i]); global sol = inpt[i]; global index = i; break)
end

found = false
i = 0
hi = undef
while !found
    acc = 0
    global i += 1
    for n in i:length(inpt)
        acc += inpt[n]
        acc > sol && break
        global found = acc == sol
        found && (global hi = n; break)
    end
end

println(maximum(inpt[i:hi]), " ", minimum(inpt[i:hi]), ", ",maximum(inpt[i:hi]) + minimum(inpt[i:hi]))
