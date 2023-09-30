# P1 - Processor One 

Simple 8-bit stack based CPU built with standard logic ICs.

## Contents

- [Project organization](#project-organization)
- [Architecture](#architecture)
- [Toolchain](#toolchain)
- [Logical design]
- [Schematics and PCB]
- [Authors]
- [License]

## Project Organization

* `benchmark` - Host system benchmark code. Implements several matrix operation on 8-bit
unsigned integers.
* `docs` - More detailed documentation on the CPU design and Instruction Set Architecture.
* `prgs` - Programs written for P1 CPU using P1 mnemonics
* `tb` - Test vector generators for ALU and branching unit. 
* `tests` - Test programs in P1 mnemonics that execute every single possible opcode in a predictable manner
* `tooling` - Mnemonics translation tool `p1asm` and CPU emulator `p1emu`

CPU logical design is done in [Logisim Evolution](https://github.com/logisim-evolution/logisim-evolution), the main circuit is 
stored in `P1.circ` file. 

CPU physical design is done in [KiCad](https://www.kicad.org/)

Tooling is written in [Rust](https://rustup.rs/)

## Architecture

P1 is a simple 8-bit, stack based CPU with 8-bit address and data buses. The main characteristic of it as it is a 
stack-based CPU, meaning that all operations are executed on a hardware stack, not on addressable registers.

CPU provides four items stack with registers `X`,`Y`,`Z` and `T`, plus a "previous X value" register `X0`. 
All operations are applied to the values of registers `X` and `Y` and operations results are stored back to register `X`. 
CPU provides wide set of stack specific operations.

For an intermediate data storage CPU provides 15 addressable registers, which can be used as a temporary data storage. 
Those registers can not be used in any other operation, except reading or updating their value. However, those 
registers could be used with branching instructions, procedure calls or data accesses as the source of address, 
thus allowing indirect operations. Some of those registers will automatically increase of decrease their values upon 
indirect call, thus allowing automatic sequential reads/write of memory.

Speaking of memory, CPU support 256 bytes of RAM **and** 256 bytes of program ROM. Program memory and data memory are in 
different address spaces, however they share the same address and data buses.

More details on the CPU instructions and behaviour details can be obtained from the [Instruction Set Architecture documentation](/docs/ISA.md).

## Toolchain