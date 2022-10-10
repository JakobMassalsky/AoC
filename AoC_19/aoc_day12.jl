using Plots
using Pkg
using UnicodePlots
using GR

mutable struct moon
    position::Vector{Int}
    velocity::Vector{Int}
end

function getKin(moon::moon)
    sum::Int = 0
    for v = abs.(moon.velocity)
        sum += v
    end
    return sum
end

function getPot(moon::moon)
    sum::Int = 0
    for v = abs.(moon.position)
        sum += v
    end
    return sum
end

getEnergy(moon::moon) = getKin(moon) * getPot(moon)

function getTotal(moons::Vector{moon})
    sum::Int = 0
    for m = moons
        sum += getEnergy(m)
    end
    return sum
end

function updateVel(moon::moon, moons::Vector{moon}, i::Int)
    adds::Int = 0
    for m = moons
        if m != moon
            adds += m.position[i] > moon.position[i] ? 1 : m.position[i] < moon.position[i] ? -1 : 0
        end
    end
    moon.velocity[i] += adds
end

function updatePos(moon::moon, i::Int)
    moon.position[i] += moon.velocity[i]
end

function getPosition(moons::Vector{moon}, i::Int)
    ret = []
    for m = moons
        push!(ret, m.position[i])
    end
    return ret
end

moon1 = moon([-13, -13, -13], [0, 0, 0])
moon2 = moon([5, -8, 3], [0, 0, 0])
moon3 = moon([-6, -10, -3], [0, 0, 0])
moon4 = moon([0, 5, -5], [0, 0, 0])

moons = [moon1, moon2, moon3, moon4]

# for t = 1:1000
#     for i = 1:4
#         updateVel(moons[i], moons)
#     end
#     for i = 1:4
#         updatePos(moons[i])
#     end
# end

xlooplength = 2
ylooplength = 2
zlooplength = 2

for z = 1:3
    currentGoTo = [moon1.position[z], moon2.position[z], moon3.position[z], moon4.position[z]]

    for i = 1:4
        updateVel(moons[i], moons, z)
    end
    for i = 1:4
        updatePos(moons[i], z)
    end

    while getPosition(moons, z) != currentGoTo
        for i = 1:4
            updateVel(moons[i], moons, z)
        end
        for i = 1:4
            updatePos(moons[i], z)
        end
        if z == 1
            global xlooplength += 1
        elseif z == 2
            global ylooplength += 1
        else
            global zlooplength += 1
        end
    end
end

println(lcm(lcm(xlooplength, ylooplength), zlooplength))

println("$xlooplength, $ylooplength, $zlooplength")
