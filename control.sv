module control (
    input clk,
    input [31:0] ir,
    input [3:0] status,
    output logic pc_rst,
    output logic inc_pc,
    output logic oe_mdr,
    output logic oe_data_reg_file,
    output logic [7:0] sel_data_reg_file,
    output logic oe_mar,
    output logic oe_addr_reg_file,
    output logic [7:0] sel_addr_reg_file,
    output logic oe_pc,
    output logic oe_alu,
    output logic [3:0] alu_op,
    output logic mem_rd,
    output logic mem_wr,
    output logic ld_reg_file,
    output logic ld_ir,
    output logic ld_pc,
    output logic ld_a,
    output logic ld_b,
    output logic ld_status,
    output logic ld_mdr,
    output logic ld_mar
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
		pc_rst,
		inc_pc,
		oe_mdr,
		oe_data_reg_file,
		sel_data_reg_file,
		oe_mar,
		oe_addr_reg_file,
		sel_addr_reg_file,
		oe_pc,
		oe_alu,
		alu_op,
		mem_rd,
		mem_wr,
		ld_reg_file,
		ld_ir,
		ld_pc,
		ld_a,
		ld_b,
		ld_status,
		ld_mdr,
		ld_mar
	} <= 0;

    case (state)
      FETCH: begin
        oe_pc  <= 1;
        mem_rd <= 1;
        ld_ir  <= 1;
      end
      STOP: ;
      default: ;
    endcase
    // fetch


  end

endmodule
