using Arrow

Arrow.write("test_ipc/test_julia.ipc", (a = Int64[1, 2, 3],); file = true)
