# Reference: https://github.com/BitVM/bitvm-js/blob/main/docs/opcodes/u8_xor.md

# Inputs
A = 0b00101010
B = 0b10100100

def f(x):
    return (x & 0b10101010) >> 1

# Algorithm
f_A = f(A)
A_even = f_A << 1
A_odd = A - A_even

f_B = f(B)
B_even = f_B << 1
B_odd = B - B_even

A_andxor_B_even = f_A + f_B
A_xor_B_even = A_andxor_B_even - (f(A_andxor_B_even) << 1)

A_andxor_B_odd = A_odd + B_odd
A_xor_B_odd = A_andxor_B_odd - (f(A_andxor_B_odd) << 1)

A_xor_B = A_xor_B_odd + (A_xor_B_even << 1)

print("expect: " + bin(A_xor_B))
print("actual: " + bin(A^B))

