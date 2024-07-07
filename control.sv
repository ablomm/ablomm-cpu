module control (
    input clk,
    input [31:0] ir,
    input [3:0] status,

    output logic mem_rd,
    output logic mem_wr,

    output logic oe_a_reg_file,
    output logic oe_b_reg_file,
    output logic ld_reg_file,
    output logic [3:0] sel_a_reg_file,
    output logic [3:0] sel_b_reg_file,
    output logic [7:0] count_a_reg_file,
    output logic [7:0] count_b_reg_file,

	output logic oe_a_ir_8,
	output logic oe_a_ir_16,
	output logic oe_b_ir_8,
	output logic oe_b_ir_16,
    output logic ld_ir,

    output logic ld_status,

    output logic oe_mdr,
    output logic ld_mdr,

    output logic oe_mar,
    output logic ld_mar,

    output logic oe_alu,
    output logic [3:0] alu_op
);

  `include "include/registers.vh"

  typedef union packed {
    logic [15:0]   one_arg;
    logic [7:0][2] two_args;
  } arg_t;


  typedef struct packed {
    logic [15:0] opcode;
    arg_t arg;
  } instruction_t;

  typedef enum {
    STOP,
    FETCH
  } states_e;

  states_e state;

  // state changes
  always_ff @(posedge clk) begin
    case (state)
      FETCH: ;  //todo
      STOP: state <= STOP;
      default: state <= FETCH;
    endcase

  end

  // outputs
  always @(state) begin
    {
	  mem_rd,
	  mem_wr,

	  oe_a_reg_file,
	  oe_b_reg_file,
	  ld_reg_file,
	  sel_a_reg_file,
	  sel_b_reg_file,
	  count_a_reg_file,
	  count_b_reg_file,

	  oe_a_ir_8,
	  oe_a_ir_16,
	  oe_b_ir_8,
	  oe_b_ir_16,
	  ld_ir,

	  ld_status,

	  oe_mdr,
	  ld_mdr,

	  oe_mar,
	  ld_mar,

	  oe_alu,
	  alu_op
  	} <= 0;

    case (state)
      FETCH: begin
        sel_b_reg_file <= `REG_PC;
		oe_b_reg_file <= 1;
        mem_rd <= 1;
        ld_ir <= 1;
      end
      STOP: ;
      default: ;
    endcase
    // fetch


  end

endmodule
