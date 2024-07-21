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
    LD   = 8'h10,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP
  } instruction_e;

  typedef struct packed {
    logic immediate;
    logic reverse;
    logic load;
    logic set_status;
  } alu_op_flags_t;

  typedef struct packed {
    alu_op_flags_t flags;
    logic [15:0]   unknown;
  } unknown_alu_op_t;

  typedef struct packed {
    alu_op_flags_t flags;
    logic [3:0] unused;
    reg_e reg_a;
    reg_e reg_b;
    reg_e reg_c;
  } alu_op_params_t;

  typedef struct packed {
    alu_op_flags_t flags;
    reg_e reg_a;
    reg_e reg_b;
    logic [7:0] immediate;
  } immediate_alu_op_params_t;

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
    unknown_alu_op_t unknown_alu_op;
    alu_op_params_t alu_op;
    immediate_alu_op_params_t alu_op_i;

    register_address_params_t   ld_params;
    register_register_params_t  ldr_params;
    register_immediate_params_t ldi_params;
    register_address_params_t   st_params;
    register_register_params_t  str_params;

    register_params_t push_params;
    register_params_t pop_params;
  } ir_params_t;

  typedef struct packed {
    cond_e condition;
    instruction_e instruction;
    ir_params_t params;
  } ir_t;

endpackage
