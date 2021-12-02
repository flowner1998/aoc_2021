mutable struct Submarine
    horizontal::Int32
    depth::Int32
end

function readInstruction(i::String, s::Submarine)
    instruction = split(i, " ")

    if instruction[1] == "forward"
        forward(s, parse(Int32, instruction[2]))
    elseif instruction[1] == "up"
        up(s, parse(Int32, instruction[2]))
    elseif instruction[1] == "down"
        down(s, parse(Int32, instruction[2]))
    else
        println("?")
    end
end

function forward(s::Submarine, a::Int32)
    s.horizontal = s.horizontal + a
end

function up(s::Submarine, a::Int32)
    s.depth = s.depth - a
end

function down(s::Submarine, a::Int32)
    s.depth = s.depth + a
end

filepath = "input"

instructions = readlines(filepath)
submarine = Submarine(0, 0)

for i in instructions
    readInstruction(i, submarine)
end

println("Answer = ", submarine.horizontal * submarine.depth)