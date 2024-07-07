package ablomm_cpu;
  typedef struct packed {
    logic negative;
    logic zero;
    logic carry;
    logic overflow;
  } status_t;

  typedef enum logic [3:0] {
    PASSA = 0,
    PASSB,
    AND,
    OR,
    XOR,
    NOT,
    ADD,
    ADDC,
    SUB,
    SUBB,
    SHL,
    SHR,
    ASHR
  } alu_op_e;

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

  typedef enum logic [3:0] {
    R0  = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    PC,
    SP,
    FP
  } reg_e;

endpackage
