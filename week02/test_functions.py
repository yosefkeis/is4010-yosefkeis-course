import sys
sys.path.append('.')
from lab02 import factorial, is_prime, reverse_string

print("factorial(5):", factorial(5))
print("factorial(0):", factorial(0))
print("is_prime(17):", is_prime(17))
print("is_prime(4):", is_prime(4))
print("is_prime(1):", is_prime(1))
print("reverse_string('hello'):", reverse_string('hello'))
print("reverse_string('Python'):", reverse_string('Python'))