inpt = "59773419794631560412886746550049210714854107066028081032096591759575145680294995770741204955183395640103527371801225795364363411455113236683168088750631442993123053909358252440339859092431844641600092736006758954422097244486920945182483159023820538645717611051770509314159895220529097322723261391627686997403783043710213655074108451646685558064317469095295303320622883691266307865809481566214524686422834824930414730886697237161697731339757655485312568793531202988525963494119232351266908405705634244498096660057021101738706453735025060225814133166491989584616948876879383198021336484629381888934600383957019607807995278899293254143523702000576897358"

# inpt = "12345678"

# include("intcodeComputer.jl")

base = [0, 1, 0, -1]

integers = parse.(Int8, split(inpt, ""))

patterns = []

for i = 1:length(integers)

    tmp::Vector{Int} = []
    x = 0
    while length(tmp) <= length(integers)
        for n = 1:i
            push!(tmp, base[x%4+1])
        end
        x += 1
    end

    popfirst!(tmp)
    while length(tmp) > length(integers)
        pop!(tmp)
    end

    push!(patterns, tmp)

end

patternsNew = []

for x = 1:length(integers)
    tmp = []
    for y = 1:length(integers)
        push!(tmp, patterns[y][x])
    end
    push!(patternsNew, tmp)
end

patterns = patternsNew
patternsNew = NaN

# println(integers)

m = 0

while m < 100
    newInpt::Array{Array{Int8}} = []
    for n = 1:length(integers)
        push!(newInpt, (patterns[n] .* integers[n]) .% 10)
    end
    # println(newInpt)
    tmp = newInpt[1]
    for i = 2:length(newInpt)
        tmp += newInpt[i]
    end
    global integers = convert.(Int8, abs.(tmp) .% 10)
    global m += 1
end
