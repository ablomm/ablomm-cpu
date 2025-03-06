package alu_pkg;

  typedef enum logic [3:0] {
    PASS = 0,
    AND,
    OR,
    XOR,
    NOT,
    ADD,
    SUB,
    NEG,
    SHL,
    SHR,
    ASHR,
    ROL,
    ROR
  } alu_op_e;

  typedef struct packed {
    logic negative;
    logic zero;
    logic carry;
    logic overflow;
  } alu_status_t;
endpackage
