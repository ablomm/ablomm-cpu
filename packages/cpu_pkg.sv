package cpu_pkg;
	import alu_pkg::*;
	import reg_pkg::*;

  typedef enum logic [3:0] {
    NONE = 0,
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
    logic [7:0] unused;
    logic status_load;
    reg_e reg_a;
    reg_e reg_b;
    reg_e reg_c;
  } and_params_t;

  typedef struct packed {
    logic [3:0] unused;
    logic status_load;
    reg_e reg_a;
    reg_e reg_b;
    logic [7:0] immediate;
  } andi_params_t;

  typedef union packed {
    and_params_t  and_params;
    andi_params_t andi_params;
  } ir_params_t;

  typedef struct packed {
    cond_e condition;
    instruction_e instruction;
    ir_params_t parameters;
  } ir_t;

endpackage
