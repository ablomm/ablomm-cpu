package alu_pkg;
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
endpackage
