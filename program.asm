movi R0, 0
movi R1, 0
movi R2, 0
movi R3, 0
jumpi 10

addi R1, R1, 1
jz R1, 20
jumpi 10

addi R2, R2, 1
jz R2, 30
jumpi 10

addi R3, R3, 1
andi R0, R3, 15
jnz R0, 10
andi R0, R2, 66
jnz R0, 10
andi R0, R1, 64
jnz R0, 10
movi R4, 9
jumpi 0