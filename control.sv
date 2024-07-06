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

    output logic ld_ir,

    output logic ld_status,

    output logic oe_mdr,
    output logic ld_mdr,

    output logic oe_mar,
    output logic ld_mar,

    output logic oe_a_pc,
    output logic oe_b_pc,
    output logic ld_pc,
    output logic rst_pc,
    output logic inc_pc,

    output logic oe_a_sp,
    output logic oe_b_sp,
    output logic ld_sp,
    output logic rst_sp,
    output logic dec_sp,

    output logic oe_a_fp,
    output logic oe_b_fp,
    output logic ld_fp,

    output logic oe_alu,
    output logic [3:0] alu_op
);


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

	  ld_ir,

	  ld_status,

	  oe_mdr,
	  ld_mdr,

	  oe_mar,
	  ld_mar,

	  oe_a_pc,
	  oe_b_pc,
	  ld_pc,
	  rst_pc,
	  inc_pc,

	  oe_a_sp,
	  oe_b_sp,
	  ld_sp,
	  rst_sp,
	  dec_sp,

	  oe_a_fp,
	  oe_b_fp,
	  ld_fp,

	  oe_alu,
	  alu_op
  	} <= 0;

    case (state)
      FETCH: begin
        oe_b_pc  <= 1;
        mem_rd <= 1;
        ld_ir  <= 1;
      end
      STOP: ;
      default: ;
    endcase
    // fetch


  end

endmodule
