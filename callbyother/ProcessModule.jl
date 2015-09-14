module ProcessModule
export process

function process(x::Int)
    for i=1:5000000
        x += 1
    end
    return x
end

end
