0x00168000 ; movi R0, 0
0x00168100 ; movi R1, 0
0x00168200 ; movi R2, 0
0x00168300 ; movi R3, 0
0x0116e60a ; jumpi 10
0x0011e101 ; addi R1, R1, 1
0x0231e614 ; jz R1, 20
0x0116e60a ; jumpi 10
0x0012e201 ; addi R2, R2, 1
0x0232e61e ; jz R2, 30
0x0116e60a ; jumpi 10
0x0013e301 ; addi R3, R3, 1
0x0003e00f ; andi R0, R3, 15
0x0330e60a ; jnz R0, 10
0x0002e042 ; andi R0, R2, 66
0x0330e60a ; jnz R0, 10
0x0001e040 ; andi R0, R1, 64
0x0330e60a ; jnz R0, 10
0x00168409 ; movi R4, 9
0x0116e600 ; jumpi 0
0, 10
0x00168400009 ; movi R4, 9
0x0116e600000 ; jumpi 0
