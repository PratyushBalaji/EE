def is_prime(num):
    for i in range(1,num//2):
        if num%(i+1) == 0:
            return False
    return True

def list_primes(n):
    i = 0
    num = 1
    while i < n:
        if is_prime(num):
            i+=1
            print(num)
        num+=1

list_primes(50)