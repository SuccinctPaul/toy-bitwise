


# pre-calculate the table of results for shifting each nibble value from 0 to 15 to the right by 1 bit.

print("Left Shifting table")
print("X(dec) \t| X(hex) \t| X(bin) \t| X>>1(dec) \t| X>>1(bin) \t| X>>1(hex)  ")

for x in range(16)[::-1]:
    v = x>>1

    bin_x = bin(x)[2:]  # Remove the '0b' prefix
    hex_x = hex(x)[2:]  # Remove the '0x' prefix

    bin_v = bin(v)[2:]  # Remove the '0b' prefix
    hex_v = hex(v)[2:]  # Remove the '0x' prefix
    print(x , "\t|\t",hex_x , "\t|\t",bin_x , "\t|\t",  v , "\t|\t",  bin_v , "\t|\t", hex_v  )



print("Right Shifting table")
print("X(dec) \t| X(hex) \t| X(bin) \t| X>>1(dec) \t| X>>1(bin) \t| X>>1(hex)  ")

for x in range(16)[::-1]:
    v = x<<1 & 0xF

    bin_x = bin(x)[2:]  # Remove the '0b' prefix
    hex_x = hex(x)[2:]  # Remove the '0x' prefix

    bin_v = bin(v)[2:]  # Remove the '0b' prefix
    hex_v = hex(v)[2:]  # Remove the '0x' prefix
    print(x , "\t|\t",hex_x , "\t|\t",bin_x , "\t|\t",  v , "\t|\t",  bin_v , "\t|\t", hex_v  )

