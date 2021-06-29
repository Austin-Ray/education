// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.
    @R2
    M=0
(LOOP)
    // Assert R0 != 0
    @R1
    D=M
    @END
    D;JEQ

    // Assert R1 != 0
    @R1
    D=M
    @END
    D;JEQ

    // Load R0 for use in multiplication.
    @R0
    D=M

    // Load R2 and add R0 to it.
    @R2
    M=M+D

    // Load R1, decrement, and store in data register.
    @R1
    M=M-1
    D=M

    @LOOP
    D;JGE
(END)
    @END
    0;JMP
