package cu_pkg;
  import alu_pkg::*;
  import reg_pkg::*;

  typedef enum logic [3:0] {
    NONE = 4'h0,
    EQ,
    NE,
    NEG,
    POS,
    VS,
    VC,
    ULT,
    UGT,
    ULE,
    UGE,
    SLT,
    SGT,
    SLE,
    SGE
  } cond_e;

  typedef enum logic [7:0] {
    NOP  = 8'h00,
    LD,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP,
    INT
    // alu ops not shown here, start with 0xf*
  } instruction_e;

  typedef struct packed {
    logic immediate;
    logic reverse;
    logic loadn;
    logic set_status;
  } alu_op_flags_t;

  typedef struct packed {
    alu_op_flags_t flags;
    reg_e reg_a;
    reg_e reg_b;
    logic [7:0] unknown;
  } unknown_alu_op_operands_t;

  typedef struct packed {
    alu_op_flags_t flags;
    reg_e reg_a;
    reg_e reg_b;
    reg_e reg_c;
    logic [3:0] unused;
  } alu_op_operands_t;

  typedef struct packed {
    alu_op_flags_t flags;
    reg_e reg_a;
    reg_e reg_b;
    logic [7:0] immediate;
  } immediate_alu_op_operands_t;

  typedef struct packed {
    reg_e reg_a;
    logic [15:0] address;
  } register_address_operands_t;

  typedef struct packed {
    reg_e reg_a;
    logic [15:0] immediate;
  } register_immediate_operands_t;

  typedef struct packed {
    reg_e reg_a;
    reg_e reg_b;
    logic signed [11:0] offset;
  } register_register_offset_operands_t;

  typedef struct packed {
    reg_e reg_a;
    logic [15:0] unused;
  } register_operands_t;

  typedef union packed {
    unknown_alu_op_operands_t unknown_alu_op;
    alu_op_operands_t alu_op;
    immediate_alu_op_operands_t alu_op_i;

    register_address_operands_t ld;
    register_register_offset_operands_t ldr;
    register_immediate_operands_t ldi;
    register_address_operands_t st;
    register_register_offset_operands_t str;

    register_operands_t push;
    register_operands_t pop;
  } ir_operands_t;

  typedef struct packed {
    cond_e condition;
    instruction_e instruction;
    ir_operands_t operands;
  } ir_t;
endpackage
