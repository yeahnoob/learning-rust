#!/usr/bin/env julia

include("ProcessModule.jl")

@everywhere using ProcessModule

M = { 0 for i=1:10 }
X = pmap(process, M)
println(X)
println("Done!")

#=
@parallel for i=1:10
    println("Thread finished with count=", process())
end
=#

