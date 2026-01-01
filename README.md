# ERIS BST-27i

> **Brusentsov Symmetric Ternary 27-trit Integer Architecture**

ERIS BST-27i is a high-fidelity CPU architecture and emulator built on the principles of balanced ternary logic. Unlike traditional binary systems that use bits (0, 1), ERIS utilizes trits ($T, 0, 1$), allowing for inherent signed arithmetic, superior radix economy, and a massive addressable space within a compact 27-trit word.The architecture serves as a modern homage to Nikolay Brusentsov’s Setun (1958), reimagined through the lens of modern RISC (Reduced Instruction Set Computer) design patterns.

## Architectural Specifications

| Feature             | Specification                       | Comparison (RV32I)               |
| ------------------- | ----------------------------------- | -------------------------------- |
| **Logic**           | Balanced Ternary ()                 | Binary ()                        |
| **Word Width**      | 27 Trits                            | 32 Bits                          |
| **GPR Count**       | 27 General Purpose Registers        | 32 Registers                     |
| **States per Word** | $3^{27} \approx 7.6 \times 10^{12}$ | $2^{32} \approx 4.2 \times 10^9$ |
| **Dynamic Range**   | $\pm 3,812,798,742,493$             | $\pm 2,147,483,648$              |
| **Address Space**   | 7.6 TB (Trit-addressable)           | 4 GB (Byte-addressable)          |
