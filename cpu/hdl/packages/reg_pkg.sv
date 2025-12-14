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
    R0 = 4'h0,
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
    STATUS,
    SP,
    LR,
    PCLINK,  // pseudo register: same as PC, except loading will also load LR to previous PC state
    PC
  } reg_e;

  typedef enum logic [1:0] {
    // least significate x bits
    LS8,
    LS16,
    LS27,
    LS32
  } reg_mask_e;

  // takes in a mask_e and converts it to the corresponding 32-bit mask
  function static logic [31:0] mask_32(input reg_mask_e mask_e);
    unique case (mask_e)
      LS8: mask_32 = 'h000000ff;
      LS16: mask_32 = 'h0000ffff;
      LS27: mask_32 = 'h00ffffff;
      LS32: mask_32 = 'hffffffff;
      default: mask_32 = 'hffffffff;
    endcase
  endfunction
endpackage
