inpt = read("AoC_20\\day04_input.txt"::AbstractString, String)

inpt = split(inpt, "\r\n\r\n")
options = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]#, "cid:"]
hex = "0123456789abcdef"
numbers = "0123456789"
colors = "amb---blu---brn---gry---grn---hzl---oth"
validCounter = 0
inpt .*= "---------"

for s = inpt
    valid = true
    for f = options
        if findnext(f, s, 1) != nothing && valid
            start = maximum(findnext(f, s, 1))+1
            if f == "byr:"
                byr = parse(Int, s[start:start+3])
                valid = byr <= 2002 && byr >= 1920
            elseif f == "iyr:"
                iyr = parse(Int, s[start:start+3])
                valid = iyr <= 2020 && iyr >= 2010
            elseif f == "eyr:"
                eyr = parse(Int, s[start:start+3])
                valid = eyr <= 2030 && eyr >= 2020
            elseif f == "hgt:"
                if s[start+2:start+3] == "in"
                    hgt = parse(Int, s[start:start+1])
                    valid = hgt <= 76 && hgt >= 59
                elseif s[start+3:start+4] == "cm"
                    hgt = parse(Int, s[start:start+2])
                    valid = hgt <= 193 && hgt >= 150
                else valid = false end
            elseif f == "hcl:"
                hcl = s[start:start+6]
                valid = hcl[1] == '#'
                for i in 2:7
                    valid = contains(hex, hcl[i]) && valid
                end
            elseif f == "ecl:"
                ecl = s[start:start+2]
                valid = contains(colors, ecl)
            elseif f == "pid:"
                pid = s[start:start+8]
                for i in 1:9
                    valid = contains(numbers, pid[i]) && valid
                end
                pidnum = 0
                valid && (pidnum = parse(Int, s[start:start+8]))
                valid = !contains(numbers, s[start+9])
                if valid
                    valid = pidnum >= 1
                end
            end
        else valid = false end
    end
    valid && (global validCounter += 1)
end
