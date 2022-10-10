include("intcodeComputer.jl")

inpt = "3,8,1005,8,311,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,28,1006,0,2,2,109,10,10,1,1,19,10,1,1103,20,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,65,1006,0,33,1,7,0,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,94,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,116,1,1002,1,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,142,2,1101,6,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,168,2,1107,7,10,1006,0,68,1,5,6,10,1,2,5,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,206,1,1008,16,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,232,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,102,1,8,253,1006,0,30,2,1,4,10,1,1008,1,10,2,1109,4,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,291,101,1,9,9,1007,9,1051,10,1005,10,15,99,109,633,104,0,104,1,21102,387508339604,1,1,21102,1,328,0,1106,0,432,21101,0,47677022988,1,21101,0,339,0,1106,0,432,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,209382822080,1,1,21102,386,1,0,1105,1,432,21101,179318123523,0,1,21102,1,397,0,1105,1,432,3,10,104,0,104,0,3,10,104,0,104,0,21102,709584904960,1,1,21101,420,0,0,1106,0,432,21102,709580444008,1,1,21102,431,1,0,1105,1,432,99,109,2,21202,-1,1,1,21102,1,40,2,21101,0,463,3,21101,0,453,0,1105,1,496,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,458,459,474,4,0,1001,458,1,458,108,4,458,10,1006,10,490,1101,0,0,458,109,-2,2106,0,0,0,109,4,2102,1,-1,495,1207,-3,0,10,1006,10,513,21102,1,0,-3,21202,-3,1,1,22102,1,-2,2,21102,1,1,3,21102,532,1,0,1106,0,537,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,560,2207,-4,-2,10,1006,10,560,21201,-4,0,-4,1106,0,628,22101,0,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,579,0,0,1105,1,537,21201,1,0,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,598,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,620,21201,-1,0,1,21101,0,620,0,106,0,495,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0"

mutable struct roboter
    x::Int
    y::Int
    directionFacing::Int
end

function paint(roboter, plane, color)
    plane[roboter.x, roboter.y] = color
    return plane
end

function turn(roboter, dir)
    if dir == 0
        roboter.directionFacing -= roboter.directionFacing != 0 ? 1 : -3
        # println("Turned left!")
    end
    if dir == 1
        roboter.directionFacing += roboter.directionFacing != 3 ? 1 : -3
        # println("Turned right!")
    end
    return roboter
end

function move(roboter, fieldsPaintedTo)

    pushField = true

    for f = fieldsPaintedTo
        if f[1] == roboter.x && f[2] == roboter.y
            pushField = false
            break
        end
    end

    pushField && push!(fieldsPaintedTo, [roboter.x, roboter.y])
    # pushField && println("Roboter has painted a total of ", length(fieldsPaintedTo), " fields!")

    # println("Roboter is facing ", roboter.directionFacing)
    if roboter.directionFacing == 1 || roboter.directionFacing == 3
        roboter.x += roboter.directionFacing == 1 ? 1 : -1
    end
    if roboter.directionFacing == 0 || roboter.directionFacing == 2
        roboter.y += roboter.directionFacing == 0 ? -1 : 1
    end
    # println("Roboter moved and is now at: x=", roboter.x, " y=", roboter.y)
    return [roboter, fieldsPaintedTo]
end

function look(roboter, plane)
    return plane[roboter.x, roboter.y]
end

function startRoboter(myRob, state, plane, fieldsPaintedTo)
    step = 1
    while state[4] == false
        input = look(myRob, plane)
        println("Camera looked at: ", input)
        state = Run(state[3], input, input, state[2], state[5])
        # println("Roboter should paint: ", state[1])
        println("Relative Base = ", state[5])
        plane = paint(myRob, plane, state[1])
        state[4] == true && break
        # println(state[3])
        state = Run(state[3], input, input, state[2], state[5])
        myRob = turn(myRob, state[1])
        field = move(myRob, fieldsPaintedTo)
        fieldsPaintedTo = field[2]
        myRob = field[1]
        # println("Intcode-State: ", state[3])
        # println("Move: ", step)
        step += 1
    end
    return [fieldsPaintedTo, plane]
end

fieldsPaintedTo = []

plane = zeros(Int8, (50, 50))
plane[5, 25] = 1

println("Matrix set up!")

prog = convertInput(inpt)

Rob = roboter(5, 25, 0)

state = [0, 1, prog, false, 0]


derOutput = startRoboter(Rob, state, plane, fieldsPaintedTo)

println(length(derOutput[1]))

for y = 1:50
    println("")
    for x = 1:50
        print(derOutput[2][x, y] == 0 ? "." : "#", " ")
    end
end
