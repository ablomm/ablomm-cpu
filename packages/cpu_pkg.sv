package cpu_pkg;
  import alu_pkg::*;
  import reg_pkg::*;

  typedef enum logic [3:0] {
    NONE = 4'h0,
    EQ,
    NE,
    LTU,
    GTU,
    LEU,
    GEU,
    LTS,
    GTS,
    LES,
    GES
  } cond_e;

  typedef enum logic [7:0] {
    NOP   = 8'h00,
    AND,
    ANDI,
    OR,
    ORI,
    XOR,
    XORI,
    NOT,
    NOTI,
    ADD,
    ADDI,
    SUB,
    SUBI,
    RSUBI,
    SHR,
    SHRI,
    ASHR,
    ASHRI,
    SHL,
    SHLI,
    LD,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP,
    MOV,
    JMP,
    JMPR
  } instruction_e;

  typedef struct packed {
    logic [6:0] unused;
    logic set_status;
    reg_e reg_a;
    reg_e reg_b;
    reg_e reg_c;
  } binary_op_params_t;

  typedef struct packed {
    logic [2:0] unused;
    logic set_status;
    reg_e reg_a;
    reg_e reg_b;
    logic [7:0] immediate;
  } binary_immediate_op_params_t;

  typedef struct packed {
    logic [10:0] unused;
    logic set_status;
    reg_e reg_a;
    reg_e reg_b;
  } unary_op_params_t;

  typedef struct packed {
    logic [6:0] unused;
    logic set_status;
    reg_e reg_a;
    logic [7:0] immediate;
  } unary_immediate_op_params_t;

  typedef struct packed {
    reg_e reg_a;
    logic [15:0] address;
  } register_address_params_t;

  typedef struct packed {
    reg_e reg_a;
    logic [15:0] immediate;
  } register_immediate_params_t;

  typedef struct packed {
    logic [11:0] unused;
    reg_e reg_a;
    reg_e reg_b;
  } register_register_params_t;

  typedef struct packed {
    logic [15:0] unused;
    reg_e reg_a;
  } register_params_t;

  typedef struct packed {
    logic [3:0]  unused;
    logic [15:0] address;
  } address_params_t;

  typedef union packed {
    binary_op_params_t and_params;
    binary_immediate_op_params_t andi_params;
    binary_op_params_t or_params;
    binary_immediate_op_params_t ori_params;
    binary_op_params_t xor_params;
    binary_immediate_op_params_t xori_params;
    unary_op_params_t not_params;
    unary_immediate_op_params_t noti_params;

    binary_op_params_t add_params;
    binary_immediate_op_params_t addi_params;
    binary_op_params_t sub_params;
    binary_immediate_op_params_t subi_params;
    binary_immediate_op_params_t rsubi_params;
    binary_op_params_t shr_params;
    binary_immediate_op_params_t shri_params;
    binary_op_params_t ashr_params;
    binary_immediate_op_params_t ashri_params;
    binary_op_params_t shl_params;
    binary_immediate_op_params_t shli_params;

    register_address_params_t   ld_params;
    register_register_params_t  ldr_params;
    register_immediate_params_t ldi_params;
    register_address_params_t   st_params;
    register_register_params_t  str_params;

    register_params_t push_params;
    register_params_t pop_params;

    register_register_params_t mov_params;
  } ir_params_t;

  typedef struct packed {
    cond_e condition;
    instruction_e instruction;
    ir_params_t params;
  } ir_t;

endpackage
