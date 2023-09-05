def recursive_factorial(n):
    if n == 1:
        return 1
    else:
        return n * recursive_factorial(n-1)

print(recursive_factorial(10))
print(len(str(recursive_factorial(34))))
