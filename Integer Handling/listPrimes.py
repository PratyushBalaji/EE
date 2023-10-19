def is_prime(num):
    for i in range(1,num//2):
        if num%(i+1) == 0:
            return False
    return True

def list_primes(n):
    i = 0
    num = 1
    arr = []
    while i < n:
        if is_prime(num):
            i+=1
            arr.append(num)
        num+=1
    return arr
