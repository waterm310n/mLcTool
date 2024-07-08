
def gcd(a,b):
    while b != 0:
        a, b = b, a % b
    return a

# 线性筛获取质数表
MX = 10**6+1
primes = [] # 质数表
is_prime = [True] * (MX) 
for i in range(2,MX):
    if is_prime[i]:
        primes.append(i)
    for p in primes:
        if p*i >= MX:
            break
        is_prime[p*i] = False
        if i % p == 0 :
            break