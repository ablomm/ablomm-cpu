package alu_pkg;

  typedef enum logic [3:0] {
    PASS = 0,
    AND,
    OR,
    XOR,
    NOT,
    ADD,
    ADDC,
    SUB,
    SUBB,
    NEG,
    SHL,
    SHR,
    ASHR
  } alu_op_e;

  typedef struct packed {
    logic negative;
    logic zero;
    logic carry;
    logic overflow;
  } alu_status_t;
endpackage
