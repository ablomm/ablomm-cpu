package reg_pkg;
  import alu_pkg::*;

  typedef enum logic {
    SUPERVISOR,
    USER
  } cpu_mode_e;

  typedef struct packed {
    alu_status_t alu_status;
    logic imask;  // interrupt mask
    cpu_mode_e mode;
  } status_t;

  typedef enum logic [3:0] {
    R0  = 4'h0,
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
    FP,
    SP,
    PC
  } reg_e;
endpackage
