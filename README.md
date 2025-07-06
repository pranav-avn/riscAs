# RISC-AS
## A simple RISC-V assembler written in Rust. 
It takes an assembly file as input, parses the instructions and labels, and outputs a binary file containing the machine code.

### Features ✨
#### RISC-V Instruction Support: Assembles a subset of RISC-V RV32I instructions, including:

- R-Type: add, sub, sll, slt, sltu, xor, srl, sra, or, and
- I-Type: addi, slti, sltiu, xori, ori, andi, slli, srli, srai
- S-Type: sb, sh, sw
- B-Type: beq, bne, blt, bge, bltu, bgeu
- U-Type: lui, auipc
- J-Type: jal

#### Label Resolution: Supports labels for branches and jumps, automatically calculating offsets.

#### Register Parsing: Handles both ABI names (e.g., ra, sp, t0) and numerical x registers (e.g., x1, x2).

#### Binary Output: Generates a .bin file containing the assembled machine code.

### How to Use 🚀
#### Prerequisites
Rust: Ensure you have Rust and Cargo installed. If not, you can install it from rust-lang.org.

#### Building the Assembler
Clone the repository:

```shell
git clone https://github.com/pranav-avn/riscAs
cd riscAs
```

#### Build the project:

```shell
cargo build --release
```

This will create an executable in the target/release/ directory.

#### Running the Assembler
To assemble an assembly file, run the compiled executable with the input assembly file and the desired output file name:

```shell
./target/release/riscv_assembler <input_assembly_file.asm> <output_binary_file_name>
```

#### Example:

Let's say you have an assembly file named example.asm:

```asm
# example.asm
addi x10, x0, 10   # a0 = 10
loop:
addi x10, x10, 1   # a0 = a0 + 1
blt x10, x11, loop # if a0 < a1, jump to loop
jal x0, end
end:
addi x10, x10, 0
```

To assemble it into program.bin, you would run:

```shell
./target/release/riscv_assembler example.asm program
```

This will generate a file named program.bin in the current directory.

#### Project Structure 📁
##### main.rs: The entry point of the assembler, handling file I/O and orchestrating the parsing process.

##### parse.rs: Contains the core logic for parsing assembly instructions, encoding them into machine code, and resolving labels.