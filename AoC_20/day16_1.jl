inpt = split(read("AoC_20\\day16_input.txt", String), "\r\n\r\n")
rules = split.(split(inpt[1], "\r\n"), ": ")
myTicket = split(split(inpt[2], "\r\n")[2], ",")
otherTickets = split.(split(inpt[3], "\r\n")[2:end], ",")
pop!(otherTickets)
Values = Vector{Int}()

for s in rules
    r = parse.(Int, split(s[2], r"-|( or )"))
    vals = append!(collect(r[1]:r[2]), collect(r[3]:r[4]))
    append!(Values, vals)
end

Values = sort(unique(Values))
acc = 0

for t in otherTickets
    for n in parse.(Int, t)
        global acc += n in Values ? 0 : n
    end
end
