import ablomm_cpu::*;

module control (
    input clk,
    input [31:0] ir,
    input wire status_t status,

    output logic mem_rd,
    output logic mem_wr,

    output logic [31:0] a_reg_mask,
    output logic [31:0] b_reg_mask,

    output logic oe_a_reg_file,
    output logic oe_b_reg_file,
    output logic ld_reg_file,
    output reg_e sel_a_reg_file,
    output reg_e sel_b_reg_file,
    output logic [7:0] count_a_reg_file,
    output logic [7:0] count_b_reg_file,

    output logic oe_a_ir,
    output logic oe_b_ir,
    output logic ld_ir,

    output logic ld_status,

    output logic oe_mdr,
    output logic ld_mdr,

    output logic oe_mar,
    output logic ld_mar,

    output logic oe_alu,
    output alu_op_e alu_op
);
  typedef enum {
    STOP,
    FETCH,
    NOP
  } states_e;

  states_e state;

  // state changes
  always_ff @(posedge clk) begin
    case (state)
      FETCH: begin
        if (satisfies_condition(ir[31:28])) begin

        end
      end
      STOP: state <= STOP;
      default: state <= FETCH;
    endcase
  end

  function static logic satisfies_condition(input cond_e [3:0] condition);
    begin
      case (condition)
        NONE: satisfies_condition = 1;
        EQ: satisfies_condition = status.zero;
        NE: satisfies_condition = !status.zero;
        LTU: satisfies_condition = !status.carry;
        GTU: satisfies_condition = status.carry && !status.zero;
        LEU: satisfies_condition = !status.carry || status.zero;
        GEU: satisfies_condition = status.carry;
        LTS: satisfies_condition = status.negative !== status.overflow;
        GTS: satisfies_condition = !status.zero && (status.negative === status.overflow);
        LES: satisfies_condition = status.zero || (status.negative !== status.overflow);
        GES: satisfies_condition = status.negative === status.overflow;
        default: satisfies_condition = 1;
      endcase
    end

  endfunction

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

	  oe_a_ir,
	  oe_b_ir,
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
      // ir <- [pc]
      // pc <- pc + 1
      FETCH: begin
        sel_b_reg_file <= PC;
        oe_b_reg_file <= 1;
        mem_rd <= 1;
        ld_ir <= 1;
        count_b_reg_file <= 8'h1;
      end
      STOP: ;
      default: ;
    endcase
    // fetch


  end

endmodule
