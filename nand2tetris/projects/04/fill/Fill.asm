// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm
//
// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed.
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(INIT_PTR)
    @ptr
    M=0

(EVENT_LOOP)
    @KBD
    D=M

    @BLACK
    D;JNE

    @WHITE
    0;JMP

(BLACK)
    // Check if ptr exceeds screen memory range
    @ptr
    D=M
    @8192 // (512x256) / 16
    D=D-A
    @INIT_PTR
    D;JEQ

    // Otherwise, update the screen.
    @ptr
    D=M

    @SCREEN
    A=A+D
    M=-1

    // Increment pointer to the next 16 characters.
    @ptr
    M=M+1

    @BLACK
    0;JMP

(WHITE)
    // Check if ptr exceeds screen memory range
    @ptr
    D=M
    @8192 // (512x256) / 16
    D=D-A
    @INIT_PTR
    D;JEQ

    // Otherwise, update the screen.
    @ptr
    D=M

    @SCREEN
    A=A+D
    M=0

    // Increment pointer to the next 16 characters.
    @ptr
    M=M+1

    @WHITE
    0;JMP
