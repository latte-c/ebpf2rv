// instruction class (3 bits)
pub const BPF_LD: u32 = 0;
pub const BPF_LDX: u32 = 1;
pub const BPF_ST: u32 = 2;
pub const BPF_STX: u32 = 3;
pub const BPF_ALU: u32 = 4;
pub const BPF_JMP: u32 = 5;
pub const BPF_JMP32: u32 = 6;
pub const BPF_ALU64: u32 = 7;

// source operand
pub const BPF_K: u32 = 0;
pub const BPF_X: u32 = 8;

// ALU instruction opcode
pub const BPF_ADD: u32 = 0x00;
pub const BPF_SUB: u32 = 0x10;
pub const BPF_MUL: u32 = 0x20;
pub const BPF_DIV: u32 = 0x30;
pub const BPF_OR: u32 = 0x40;
pub const BPF_AND: u32 = 0x50;
pub const BPF_LSH: u32 = 0x60;
pub const BPF_RSH: u32 = 0x70;
pub const BPF_NEG: u32 = 0x80;
pub const BPF_MOD: u32 = 0x90;
pub const BPF_XOR: u32 = 0xa0;
pub const BPF_MOV: u32 = 0xb0;
pub const BPF_ARSH: u32 = 0xc0;
pub const BPF_END: u32 = 0xd0;

// JMP instruction opcode
pub const BPF_JA: u32 = 0x00;
pub const BPF_JEQ: u32 = 0x10;
pub const BPF_JGT: u32 = 0x20;
pub const BPF_JGE: u32 = 0x30;
pub const BPF_JSET: u32 = 0x40;
pub const BPF_JNE: u32 = 0x50;
pub const BPF_JSGT: u32 = 0x60;
pub const BPF_JSGE: u32 = 0x70;
pub const BPF_CALL: u32 = 0x80;
pub const BPF_EXIT: u32 = 0x90;
pub const BPF_JLT: u32 = 0xa0;
pub const BPF_JLE: u32 = 0xb0;
pub const BPF_JSLT: u32 = 0xc0;
pub const BPF_JSLE: u32 = 0xd0;

// size modifier
pub const BPF_W: u32 = 0;
pub const BPF_H: u32 = 8;
pub const BPF_B: u32 = 16;
pub const BPF_DW: u32 = 24;

// mode modifier
pub const BPF_IMM: u32 = 0x00;
pub const BPF_ABS: u32 = 0x20;
pub const BPF_IND: u32 = 0x40;
pub const BPF_MEM: u32 = 0x60;
pub const BPF_ATOMIC: u32 = 0xc0;

// TODO
pub const BPF_LEN: u32 = 128;
pub const BPF_MSH: u32 = 160;
pub const BPF_MAXINSNS: u32 = 4096;
pub const BPF_XADD: u32 = 192;
pub const BPF_TO_LE: u32 = 0;
pub const BPF_TO_BE: u32 = 8;
pub const BPF_FROM_LE: u32 = 0;
pub const BPF_FROM_BE: u32 = 8;
pub const BPF_FETCH: u32 = 1;
pub const BPF_XCHG: u32 = 225;
pub const BPF_CMPXCHG: u32 = 241;
pub const BPF_F_ALLOW_OVERRIDE: u32 = 1;
pub const BPF_F_ALLOW_MULTI: u32 = 2;
pub const BPF_F_REPLACE: u32 = 4;
pub const BPF_F_STRICT_ALIGNMENT: u32 = 1;
pub const BPF_F_ANY_ALIGNMENT: u32 = 2;
pub const BPF_F_TEST_RND_HI32: u32 = 4;
pub const BPF_F_TEST_STATE_FREQ: u32 = 8;
pub const BPF_F_SLEEPABLE: u32 = 16;
pub const BPF_PSEUDO_MAP_FD: u32 = 1;
pub const BPF_PSEUDO_MAP_IDX: u32 = 5;
pub const BPF_PSEUDO_MAP_VALUE: u32 = 2;
pub const BPF_PSEUDO_MAP_IDX_VALUE: u32 = 6;
pub const BPF_PSEUDO_BTF_ID: u32 = 3;
pub const BPF_PSEUDO_FUNC: u32 = 4;
pub const BPF_PSEUDO_CALL: u32 = 1;
pub const BPF_PSEUDO_KFUNC_CALL: u32 = 2;
pub const BPF_F_QUERY_EFFECTIVE: u32 = 1;
pub const BPF_F_TEST_RUN_ON_CPU: u32 = 1;
pub const BPF_BUILD_ID_SIZE: u32 = 20;
pub const BPF_OBJ_NAME_LEN: u32 = 16;
pub const BPF_TAG_SIZE: u32 = 8;

pub const ALU_K_ADD: u8 = (BPF_ALU | BPF_K | BPF_ADD) as u8;
pub const ALU_X_ADD: u8 = (BPF_ALU | BPF_X | BPF_ADD) as u8;
pub const ALU_K_SUB: u8 = (BPF_ALU | BPF_K | BPF_SUB) as u8;
pub const ALU_X_SUB: u8 = (BPF_ALU | BPF_X | BPF_SUB) as u8;
pub const ALU_K_MUL: u8 = (BPF_ALU | BPF_K | BPF_MUL) as u8;
pub const ALU_X_MUL: u8 = (BPF_ALU | BPF_X | BPF_MUL) as u8;
pub const ALU_K_DIV: u8 = (BPF_ALU | BPF_K | BPF_DIV) as u8;
pub const ALU_X_DIV: u8 = (BPF_ALU | BPF_X | BPF_DIV) as u8;
pub const ALU_K_OR: u8 = (BPF_ALU | BPF_K | BPF_OR) as u8;
pub const ALU_X_OR: u8 = (BPF_ALU | BPF_X | BPF_OR) as u8;
pub const ALU_K_AND: u8 = (BPF_ALU | BPF_K | BPF_AND) as u8;
pub const ALU_X_AND: u8 = (BPF_ALU | BPF_X | BPF_AND) as u8;
pub const ALU_K_LSH: u8 = (BPF_ALU | BPF_K | BPF_LSH) as u8;
pub const ALU_X_LSH: u8 = (BPF_ALU | BPF_X | BPF_LSH) as u8;
pub const ALU_K_RSH: u8 = (BPF_ALU | BPF_K | BPF_RSH) as u8;
pub const ALU_X_RSH: u8 = (BPF_ALU | BPF_X | BPF_RSH) as u8;
pub const ALU_K_NEG: u8 = (BPF_ALU | BPF_K | BPF_NEG) as u8;
pub const ALU_X_NEG: u8 = (BPF_ALU | BPF_X | BPF_NEG) as u8;
pub const ALU_K_MOD: u8 = (BPF_ALU | BPF_K | BPF_MOD) as u8;
pub const ALU_X_MOD: u8 = (BPF_ALU | BPF_X | BPF_MOD) as u8;
pub const ALU_K_XOR: u8 = (BPF_ALU | BPF_K | BPF_XOR) as u8;
pub const ALU_X_XOR: u8 = (BPF_ALU | BPF_X | BPF_XOR) as u8;
pub const ALU_K_MOV: u8 = (BPF_ALU | BPF_K | BPF_MOV) as u8;
pub const ALU_X_MOV: u8 = (BPF_ALU | BPF_X | BPF_MOV) as u8;
pub const ALU_K_ARSH: u8 = (BPF_ALU | BPF_K | BPF_ARSH) as u8;
pub const ALU_X_ARSH: u8 = (BPF_ALU | BPF_X | BPF_ARSH) as u8;
pub const ALU_K_END: u8 = (BPF_ALU | BPF_K | BPF_END) as u8;
pub const ALU_X_END: u8 = (BPF_ALU | BPF_X | BPF_END) as u8;

pub const ALU64_K_ADD: u8 = (BPF_ALU64 | BPF_K | BPF_ADD) as u8;
pub const ALU64_X_ADD: u8 = (BPF_ALU64 | BPF_X | BPF_ADD) as u8;
pub const ALU64_K_SUB: u8 = (BPF_ALU64 | BPF_K | BPF_SUB) as u8;
pub const ALU64_X_SUB: u8 = (BPF_ALU64 | BPF_X | BPF_SUB) as u8;
pub const ALU64_K_MUL: u8 = (BPF_ALU64 | BPF_K | BPF_MUL) as u8;
pub const ALU64_X_MUL: u8 = (BPF_ALU64 | BPF_X | BPF_MUL) as u8;
pub const ALU64_K_DIV: u8 = (BPF_ALU64 | BPF_K | BPF_DIV) as u8;
pub const ALU64_X_DIV: u8 = (BPF_ALU64 | BPF_X | BPF_DIV) as u8;
pub const ALU64_K_OR: u8 = (BPF_ALU64 | BPF_K | BPF_OR) as u8;
pub const ALU64_X_OR: u8 = (BPF_ALU64 | BPF_X | BPF_OR) as u8;
pub const ALU64_K_AND: u8 = (BPF_ALU64 | BPF_K | BPF_AND) as u8;
pub const ALU64_X_AND: u8 = (BPF_ALU64 | BPF_X | BPF_AND) as u8;
pub const ALU64_K_LSH: u8 = (BPF_ALU64 | BPF_K | BPF_LSH) as u8;
pub const ALU64_X_LSH: u8 = (BPF_ALU64 | BPF_X | BPF_LSH) as u8;
pub const ALU64_K_RSH: u8 = (BPF_ALU64 | BPF_K | BPF_RSH) as u8;
pub const ALU64_X_RSH: u8 = (BPF_ALU64 | BPF_X | BPF_RSH) as u8;
pub const ALU64_K_NEG: u8 = (BPF_ALU64 | BPF_K | BPF_NEG) as u8;
pub const ALU64_X_NEG: u8 = (BPF_ALU64 | BPF_X | BPF_NEG) as u8;
pub const ALU64_K_MOD: u8 = (BPF_ALU64 | BPF_K | BPF_MOD) as u8;
pub const ALU64_X_MOD: u8 = (BPF_ALU64 | BPF_X | BPF_MOD) as u8;
pub const ALU64_K_XOR: u8 = (BPF_ALU64 | BPF_K | BPF_XOR) as u8;
pub const ALU64_X_XOR: u8 = (BPF_ALU64 | BPF_X | BPF_XOR) as u8;
pub const ALU64_K_MOV: u8 = (BPF_ALU64 | BPF_K | BPF_MOV) as u8;
pub const ALU64_X_MOV: u8 = (BPF_ALU64 | BPF_X | BPF_MOV) as u8;
pub const ALU64_K_ARSH: u8 = (BPF_ALU64 | BPF_K | BPF_ARSH) as u8;
pub const ALU64_X_ARSH: u8 = (BPF_ALU64 | BPF_X | BPF_ARSH) as u8;
pub const ALU64_K_END: u8 = (BPF_ALU64 | BPF_K | BPF_END) as u8;
pub const ALU64_X_END: u8 = (BPF_ALU64 | BPF_X | BPF_END) as u8;

pub const JMP_K_JA: u8 = (BPF_JMP | BPF_K | BPF_JA) as u8;
pub const JMP_X_JA: u8 = (BPF_JMP | BPF_X | BPF_JA) as u8;
pub const JMP_K_JEQ: u8 = (BPF_JMP | BPF_K | BPF_JEQ) as u8;
pub const JMP_X_JEQ: u8 = (BPF_JMP | BPF_X | BPF_JEQ) as u8;
pub const JMP_K_JGT: u8 = (BPF_JMP | BPF_K | BPF_JGT) as u8;
pub const JMP_X_JGT: u8 = (BPF_JMP | BPF_X | BPF_JGT) as u8;
pub const JMP_K_JGE: u8 = (BPF_JMP | BPF_K | BPF_JGE) as u8;
pub const JMP_X_JGE: u8 = (BPF_JMP | BPF_X | BPF_JGE) as u8;
pub const JMP_K_JSET: u8 = (BPF_JMP | BPF_K | BPF_JSET) as u8;
pub const JMP_X_JSET: u8 = (BPF_JMP | BPF_X | BPF_JSET) as u8;
pub const JMP_K_JNE: u8 = (BPF_JMP | BPF_K | BPF_JNE) as u8;
pub const JMP_X_JNE: u8 = (BPF_JMP | BPF_X | BPF_JNE) as u8;
pub const JMP_K_JSGT: u8 = (BPF_JMP | BPF_K | BPF_JSGT) as u8;
pub const JMP_X_JSGT: u8 = (BPF_JMP | BPF_X | BPF_JSGT) as u8;
pub const JMP_K_JSGE: u8 = (BPF_JMP | BPF_K | BPF_JSGE) as u8;
pub const JMP_X_JSGE: u8 = (BPF_JMP | BPF_X | BPF_JSGE) as u8;
pub const JMP_K_CALL: u8 = (BPF_JMP | BPF_K | BPF_CALL) as u8;
pub const JMP_X_CALL: u8 = (BPF_JMP | BPF_X | BPF_CALL) as u8;
pub const JMP_K_EXIT: u8 = (BPF_JMP | BPF_K | BPF_EXIT) as u8;
pub const JMP_X_EXIT: u8 = (BPF_JMP | BPF_X | BPF_EXIT) as u8;
pub const JMP_K_JLT: u8 = (BPF_JMP | BPF_K | BPF_JLT) as u8;
pub const JMP_X_JLT: u8 = (BPF_JMP | BPF_X | BPF_JLT) as u8;
pub const JMP_K_JLE: u8 = (BPF_JMP | BPF_K | BPF_JLE) as u8;
pub const JMP_X_JLE: u8 = (BPF_JMP | BPF_X | BPF_JLE) as u8;
pub const JMP_K_JSLT: u8 = (BPF_JMP | BPF_K | BPF_JSLT) as u8;
pub const JMP_X_JSLT: u8 = (BPF_JMP | BPF_X | BPF_JSLT) as u8;
pub const JMP_K_JSLE: u8 = (BPF_JMP | BPF_K | BPF_JSLE) as u8;
pub const JMP_X_JSLE: u8 = (BPF_JMP | BPF_X | BPF_JSLE) as u8;

pub const JMP32_K_JA: u8 = (BPF_JMP32 | BPF_K | BPF_JA) as u8;
pub const JMP32_X_JA: u8 = (BPF_JMP32 | BPF_X | BPF_JA) as u8;
pub const JMP32_K_JEQ: u8 = (BPF_JMP32 | BPF_K | BPF_JEQ) as u8;
pub const JMP32_X_JEQ: u8 = (BPF_JMP32 | BPF_X | BPF_JEQ) as u8;
pub const JMP32_K_JGT: u8 = (BPF_JMP32 | BPF_K | BPF_JGT) as u8;
pub const JMP32_X_JGT: u8 = (BPF_JMP32 | BPF_X | BPF_JGT) as u8;
pub const JMP32_K_JGE: u8 = (BPF_JMP32 | BPF_K | BPF_JGE) as u8;
pub const JMP32_X_JGE: u8 = (BPF_JMP32 | BPF_X | BPF_JGE) as u8;
pub const JMP32_K_JSET: u8 = (BPF_JMP32 | BPF_K | BPF_JSET) as u8;
pub const JMP32_X_JSET: u8 = (BPF_JMP32 | BPF_X | BPF_JSET) as u8;
pub const JMP32_K_JNE: u8 = (BPF_JMP32 | BPF_K | BPF_JNE) as u8;
pub const JMP32_X_JNE: u8 = (BPF_JMP32 | BPF_X | BPF_JNE) as u8;
pub const JMP32_K_JSGT: u8 = (BPF_JMP32 | BPF_K | BPF_JSGT) as u8;
pub const JMP32_X_JSGT: u8 = (BPF_JMP32 | BPF_X | BPF_JSGT) as u8;
pub const JMP32_K_JSGE: u8 = (BPF_JMP32 | BPF_K | BPF_JSGE) as u8;
pub const JMP32_X_JSGE: u8 = (BPF_JMP32 | BPF_X | BPF_JSGE) as u8;
pub const JMP32_K_CALL: u8 = (BPF_JMP32 | BPF_K | BPF_CALL) as u8;
pub const JMP32_X_CALL: u8 = (BPF_JMP32 | BPF_X | BPF_CALL) as u8;
pub const JMP32_K_EXIT: u8 = (BPF_JMP32 | BPF_K | BPF_EXIT) as u8;
pub const JMP32_X_EXIT: u8 = (BPF_JMP32 | BPF_X | BPF_EXIT) as u8;
pub const JMP32_K_JLT: u8 = (BPF_JMP32 | BPF_K | BPF_JLT) as u8;
pub const JMP32_X_JLT: u8 = (BPF_JMP32 | BPF_X | BPF_JLT) as u8;
pub const JMP32_K_JLE: u8 = (BPF_JMP32 | BPF_K | BPF_JLE) as u8;
pub const JMP32_X_JLE: u8 = (BPF_JMP32 | BPF_X | BPF_JLE) as u8;
pub const JMP32_K_JSLT: u8 = (BPF_JMP32 | BPF_K | BPF_JSLT) as u8;
pub const JMP32_X_JSLT: u8 = (BPF_JMP32 | BPF_X | BPF_JSLT) as u8;
pub const JMP32_K_JSLE: u8 = (BPF_JMP32 | BPF_K | BPF_JSLE) as u8;
pub const JMP32_X_JSLE: u8 = (BPF_JMP32 | BPF_X | BPF_JSLE) as u8;

pub const LD_IMM_B: u8 = (BPF_LD | BPF_IMM | BPF_B) as u8;
pub const LD_IMM_H: u8 = (BPF_LD | BPF_IMM | BPF_H) as u8;
pub const LD_IMM_W: u8 = (BPF_LD | BPF_IMM | BPF_W) as u8;
pub const LD_IMM_DW: u8 = (BPF_LD | BPF_IMM | BPF_DW) as u8;
pub const LD_ABS_B: u8 = (BPF_LD | BPF_ABS | BPF_B) as u8;
pub const LD_ABS_H: u8 = (BPF_LD | BPF_ABS | BPF_H) as u8;
pub const LD_ABS_W: u8 = (BPF_LD | BPF_ABS | BPF_W) as u8;
pub const LD_ABS_DW: u8 = (BPF_LD | BPF_ABS | BPF_DW) as u8;
pub const LD_IND_B: u8 = (BPF_LD | BPF_IND | BPF_B) as u8;
pub const LD_IND_H: u8 = (BPF_LD | BPF_IND | BPF_H) as u8;
pub const LD_IND_W: u8 = (BPF_LD | BPF_IND | BPF_W) as u8;
pub const LD_IND_DW: u8 = (BPF_LD | BPF_IND | BPF_DW) as u8;
pub const LD_MEM_B: u8 = (BPF_LD | BPF_MEM | BPF_B) as u8;
pub const LD_MEM_H: u8 = (BPF_LD | BPF_MEM | BPF_H) as u8;
pub const LD_MEM_W: u8 = (BPF_LD | BPF_MEM | BPF_W) as u8;
pub const LD_MEM_DW: u8 = (BPF_LD | BPF_MEM | BPF_DW) as u8;
pub const LD_XADD_B: u8 = (BPF_LD | BPF_XADD | BPF_B) as u8;
pub const LD_XADD_H: u8 = (BPF_LD | BPF_XADD | BPF_H) as u8;
pub const LD_XADD_W: u8 = (BPF_LD | BPF_XADD | BPF_W) as u8;
pub const LD_XADD_DW: u8 = (BPF_LD | BPF_XADD | BPF_DW) as u8;

pub const LDX_IMM_B: u8 = (BPF_LDX | BPF_IMM | BPF_B) as u8;
pub const LDX_IMM_H: u8 = (BPF_LDX | BPF_IMM | BPF_H) as u8;
pub const LDX_IMM_W: u8 = (BPF_LDX | BPF_IMM | BPF_W) as u8;
pub const LDX_IMM_DW: u8 = (BPF_LDX | BPF_IMM | BPF_DW) as u8;
pub const LDX_ABS_B: u8 = (BPF_LDX | BPF_ABS | BPF_B) as u8;
pub const LDX_ABS_H: u8 = (BPF_LDX | BPF_ABS | BPF_H) as u8;
pub const LDX_ABS_W: u8 = (BPF_LDX | BPF_ABS | BPF_W) as u8;
pub const LDX_ABS_DW: u8 = (BPF_LDX | BPF_ABS | BPF_DW) as u8;
pub const LDX_IND_B: u8 = (BPF_LDX | BPF_IND | BPF_B) as u8;
pub const LDX_IND_H: u8 = (BPF_LDX | BPF_IND | BPF_H) as u8;
pub const LDX_IND_W: u8 = (BPF_LDX | BPF_IND | BPF_W) as u8;
pub const LDX_IND_DW: u8 = (BPF_LDX | BPF_IND | BPF_DW) as u8;
pub const LDX_MEM_B: u8 = (BPF_LDX | BPF_MEM | BPF_B) as u8;
pub const LDX_MEM_H: u8 = (BPF_LDX | BPF_MEM | BPF_H) as u8;
pub const LDX_MEM_W: u8 = (BPF_LDX | BPF_MEM | BPF_W) as u8;
pub const LDX_MEM_DW: u8 = (BPF_LDX | BPF_MEM | BPF_DW) as u8;
pub const LDX_XADD_B: u8 = (BPF_LDX | BPF_XADD | BPF_B) as u8;
pub const LDX_XADD_H: u8 = (BPF_LDX | BPF_XADD | BPF_H) as u8;
pub const LDX_XADD_W: u8 = (BPF_LDX | BPF_XADD | BPF_W) as u8;
pub const LDX_XADD_DW: u8 = (BPF_LDX | BPF_XADD | BPF_DW) as u8;

pub const ST_IMM_B: u8 = (BPF_ST | BPF_IMM | BPF_B) as u8;
pub const ST_IMM_H: u8 = (BPF_ST | BPF_IMM | BPF_H) as u8;
pub const ST_IMM_W: u8 = (BPF_ST | BPF_IMM | BPF_W) as u8;
pub const ST_IMM_DW: u8 = (BPF_ST | BPF_IMM | BPF_DW) as u8;
pub const ST_ABS_B: u8 = (BPF_ST | BPF_ABS | BPF_B) as u8;
pub const ST_ABS_H: u8 = (BPF_ST | BPF_ABS | BPF_H) as u8;
pub const ST_ABS_W: u8 = (BPF_ST | BPF_ABS | BPF_W) as u8;
pub const ST_ABS_DW: u8 = (BPF_ST | BPF_ABS | BPF_DW) as u8;
pub const ST_IND_B: u8 = (BPF_ST | BPF_IND | BPF_B) as u8;
pub const ST_IND_H: u8 = (BPF_ST | BPF_IND | BPF_H) as u8;
pub const ST_IND_W: u8 = (BPF_ST | BPF_IND | BPF_W) as u8;
pub const ST_IND_DW: u8 = (BPF_ST | BPF_IND | BPF_DW) as u8;
pub const ST_MEM_B: u8 = (BPF_ST | BPF_MEM | BPF_B) as u8;
pub const ST_MEM_H: u8 = (BPF_ST | BPF_MEM | BPF_H) as u8;
pub const ST_MEM_W: u8 = (BPF_ST | BPF_MEM | BPF_W) as u8;
pub const ST_MEM_DW: u8 = (BPF_ST | BPF_MEM | BPF_DW) as u8;
pub const ST_XADD_B: u8 = (BPF_ST | BPF_XADD | BPF_B) as u8;
pub const ST_XADD_H: u8 = (BPF_ST | BPF_XADD | BPF_H) as u8;
pub const ST_XADD_W: u8 = (BPF_ST | BPF_XADD | BPF_W) as u8;
pub const ST_XADD_DW: u8 = (BPF_ST | BPF_XADD | BPF_DW) as u8;

pub const STX_IMM_B: u8 = (BPF_STX | BPF_IMM | BPF_B) as u8;
pub const STX_IMM_H: u8 = (BPF_STX | BPF_IMM | BPF_H) as u8;
pub const STX_IMM_W: u8 = (BPF_STX | BPF_IMM | BPF_W) as u8;
pub const STX_IMM_DW: u8 = (BPF_STX | BPF_IMM | BPF_DW) as u8;
pub const STX_ABS_B: u8 = (BPF_STX | BPF_ABS | BPF_B) as u8;
pub const STX_ABS_H: u8 = (BPF_STX | BPF_ABS | BPF_H) as u8;
pub const STX_ABS_W: u8 = (BPF_STX | BPF_ABS | BPF_W) as u8;
pub const STX_ABS_DW: u8 = (BPF_STX | BPF_ABS | BPF_DW) as u8;
pub const STX_IND_B: u8 = (BPF_STX | BPF_IND | BPF_B) as u8;
pub const STX_IND_H: u8 = (BPF_STX | BPF_IND | BPF_H) as u8;
pub const STX_IND_W: u8 = (BPF_STX | BPF_IND | BPF_W) as u8;
pub const STX_IND_DW: u8 = (BPF_STX | BPF_IND | BPF_DW) as u8;
pub const STX_MEM_B: u8 = (BPF_STX | BPF_MEM | BPF_B) as u8;
pub const STX_MEM_H: u8 = (BPF_STX | BPF_MEM | BPF_H) as u8;
pub const STX_MEM_W: u8 = (BPF_STX | BPF_MEM | BPF_W) as u8;
pub const STX_MEM_DW: u8 = (BPF_STX | BPF_MEM | BPF_DW) as u8;
pub const STX_XADD_B: u8 = (BPF_STX | BPF_XADD | BPF_B) as u8;
pub const STX_XADD_H: u8 = (BPF_STX | BPF_XADD | BPF_H) as u8;
pub const STX_XADD_W: u8 = (BPF_STX | BPF_XADD | BPF_W) as u8;
pub const STX_XADD_DW: u8 = (BPF_STX | BPF_XADD | BPF_DW) as u8;

pub const BPF_REG_R0: u8 = 0;
pub const BPF_REG_R1: u8 = 1;
pub const BPF_REG_R2: u8 = 2;
pub const BPF_REG_R3: u8 = 3;
pub const BPF_REG_R4: u8 = 4;
pub const BPF_REG_R5: u8 = 5;
pub const BPF_REG_R6: u8 = 6;
pub const BPF_REG_R7: u8 = 7;
pub const BPF_REG_R8: u8 = 8;
pub const BPF_REG_R9: u8 = 9;
pub const BPF_REG_FP: u8 = 10;
pub const BPF_MAX_REGS: usize = 11;

// RISC-V constants
pub const RV_REG_ZERO: u8 = 0;
pub const RV_REG_RA: u8 = 1;
pub const RV_REG_SP: u8 = 2;
pub const RV_REG_T0: u8 = 5;
pub const RV_REG_T1: u8 = 6;
pub const RV_REG_T2: u8 = 7;
pub const RV_REG_FP: u8 = 8;
pub const RV_REG_S1: u8 = 9;
pub const RV_REG_A0: u8 = 10;
pub const RV_REG_A1: u8 = 11;
pub const RV_REG_A2: u8 = 12;
pub const RV_REG_A3: u8 = 13;
pub const RV_REG_A4: u8 = 14;
pub const RV_REG_A5: u8 = 15;
pub const RV_REG_S2: u8 = 18;
pub const RV_REG_S3: u8 = 19;
pub const RV_REG_S4: u8 = 20;
pub const RV_REG_S5: u8 = 21;
