def fibonacci(n):
    
    x = 0
    y = 1

    print(0)
    print(1)
    for i in range(n-2):
        z = x+y
        print(z)
        x = y
        y = z

fibonacci(50)