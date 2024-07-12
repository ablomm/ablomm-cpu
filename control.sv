import cpu_pkg::*;
import alu_pkg::*;
import reg_pkg::*;

module control (
    input clk,
    input wire start,

    input wire ir_t ir,
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
    output reg_e sel_in_reg_file,
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
    NOP,
    AND,
    ANDI,
    OR,
    ORI,
    XOR,
    XORI,
    NOT,
    NOTI,
    ADD,
    ADDI,
    SUB,
    SUBI,
    RSUBI,
    SHR,
    SHRI,
    ASHR,
    ASHRI,
    SHL,
    SHLI,
	LD,
	LDR,
	LDI,
	ST,
	STR
  } states_e;

  states_e state;

  always @(posedge start) begin
    state <= FETCH;
  end

  // state changes
  always_ff @(posedge clk) begin
    case (state)
      FETCH: begin
        if (satisfies_condition(ir.condition, status)) begin
          case (ir.instruction)
            cpu_pkg::NOP: ;
            cpu_pkg::AND: state <= AND;
            cpu_pkg::ANDI: state <= ANDI;
            cpu_pkg::OR: state <= OR;
            cpu_pkg::ORI: state <= ORI;
            cpu_pkg::XOR: state <= XOR;
            cpu_pkg::XORI: state <= XORI;
            cpu_pkg::NOT: state <= NOT;
            cpu_pkg::NOTI: state <= NOTI;
            cpu_pkg::ADD: state <= ADD;
            cpu_pkg::ADDI: state <= ADDI;
            cpu_pkg::SUB: state <= SUB;
            cpu_pkg::SUBI: state <= SUBI;
            cpu_pkg::RSUBI: state <= RSUBI;
            cpu_pkg::SHR: state <= SHR;
            cpu_pkg::SHRI: state <= SHRI;
            cpu_pkg::SHR: state <= ASHR;
            cpu_pkg::ASHRI: state <= ASHRI;
            cpu_pkg::SHL: state <= SHL;
            cpu_pkg::SHLI: state <= SHLI;
            cpu_pkg::LD: state <= LD;
            cpu_pkg::LDR: state <= LDR;
            cpu_pkg::LDI: state <= LDI;
            cpu_pkg::ST: state <= ST;
            cpu_pkg::STR: state <= STR;
            default: ;
          endcase

        end
      end
      STOP: state <= STOP;
      default: state <= FETCH;
    endcase
  end

  function static logic satisfies_condition(input cond_e condition, input status_t status);
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

      // ir <- *(pc++)
      FETCH: begin
        sel_b_reg_file <= reg_pkg::PC;
        oe_b_reg_file <= 1;
        mem_rd <= 1;
        ld_ir <= 1;
        count_b_reg_file <= 8'h1;
      end

      // reg_a <- reg_b & reb_c
      AND:
      do_binary_operation(alu_pkg::AND, ir.params.and_params.set_status, ir.params.and_params.reg_a,
                          ir.params.and_params.reg_b, ir.params.and_params.reg_c);

      // reg_a <- reg_b & immediate
      ANDI:
      do_binary_operation_i(alu_pkg::AND, ir.params.andi_params.set_status,
                            ir.params.andi_params.reg_a, ir.params.andi_params.reg_b);

      // reg_a <- reg_b | reg_c
      OR:
      do_binary_operation(alu_pkg::OR, ir.params.or_params.set_status, ir.params.or_params.reg_a,
                          ir.params.or_params.reg_b, ir.params.or_params.reg_c);

      // reg_a <- reg_b | immediate
      ORI:
      do_binary_operation_i(alu_pkg::OR, ir.params.ori_params.set_status,
                            ir.params.ori_params.reg_a, ir.params.ori_params.reg_b);

      // reg_a <- reg_b ^ reg_c
      XOR:
      do_binary_operation(alu_pkg::XOR, ir.params.xor_params.set_status, ir.params.xor_params.reg_a,
                          ir.params.xor_params.reg_b, ir.params.xor_params.reg_c);

      // reg_a <- reg_b ^ immediate
      XORI:
      do_binary_operation_i(alu_pkg::OR, ir.params.xori_params.set_status,
                            ir.params.xori_params.reg_a, ir.params.xori_params.reg_b);

      // reg_a <- ~reg_b
      NOT:
      do_unary_operation(alu_pkg::NOT, ir.params.not_params.set_status, ir.params.not_params.reg_a,
                         ir.params.not_params.reg_b);

      // reg_a <- ~immediate
      NOTI:
      do_unary_operation_i(alu_pkg::NOT, ir.params.noti_params.set_status,
                           ir.params.noti_params.reg_a);

      // reg_a <- reg_b + reg_c
      ADD:
      do_binary_operation(alu_pkg::ADD, ir.params.add_params.set_status, ir.params.add_params.reg_a,
                          ir.params.add_params.reg_b, ir.params.add_params.reg_c);

      // reg_a <- reg_b + immediate
      ADDI:
      do_binary_operation_i(alu_pkg::ADD, ir.params.addi_params.set_status,
                            ir.params.addi_params.reg_a, ir.params.addi_params.reg_b);

      // reg_a <- reg_b - reg_c
      SUB:
      do_binary_operation(alu_pkg::SUB, ir.params.sub_params.set_status, ir.params.sub_params.reg_a,
                          ir.params.sub_params.reg_b, ir.params.sub_params.reg_c);

      // reg_a <- reg_b - immediate
      SUBI:
      do_binary_operation_i(alu_pkg::SUB, ir.params.subi_params.set_status,
                            ir.params.subi_params.reg_a, ir.params.subi_params.reg_b);

      // reg_a <- immediate - reg_b
      RSUBI:
      do_reverse_binary_operation_i(alu_pkg::SUB, ir.params.subi_params.set_status,
                                    ir.params.subi_params.reg_a, ir.params.subi_params.reg_b);

      // reg_a <- reg_b >> reg_c
      SHR:
      do_binary_operation(alu_pkg::SHR, ir.params.shr_params.set_status, ir.params.shr_params.reg_a,
                          ir.params.shr_params.reg_b, ir.params.shr_params.reg_c);

      // reg_a <- reg_b >> immediate
      SHRI:
      do_binary_operation_i(alu_pkg::SHR, ir.params.shri_params.set_status,
                            ir.params.shri_params.reg_a, ir.params.shri_params.reg_b);

      // reg_a <- reg_b >>> reg_c
      ASHR:
      do_binary_operation(alu_pkg::ASHR, ir.params.ashr_params.set_status,
                          ir.params.ashr_params.reg_a, ir.params.ashr_params.reg_b,
                          ir.params.ashr_params.reg_c);

      // reg_a <- reg_b >>> immediate
      ASHRI:
      do_binary_operation_i(alu_pkg::ASHR, ir.params.ashri_params.set_status,
                            ir.params.ashri_params.reg_a, ir.params.ashri_params.reg_b);

      // reg_a <- reg_b << reg_c
      SHL:
      do_binary_operation(alu_pkg::SHL, ir.params.shl_params.set_status, ir.params.shl_params.reg_a,
                          ir.params.shl_params.reg_b, ir.params.shl_params.reg_c);

      // reg_a <- reg_b << immediate
      SHLI:
      do_binary_operation_i(alu_pkg::SHR, ir.params.shli_params.set_status,
                            ir.params.shli_params.reg_a, ir.params.shli_params.reg_b);

      // reg_a <- *address
      LD: begin
        oe_b_ir <= 1;
        b_reg_mask <= 32'hff;
        mem_rd <= 1;
        sel_in_reg_file <= ir.params.ld_params.reg_a;
        ld_reg_file <= 1;
      end

      // reg_a <- *reg_b
      LDR: begin
        sel_b_reg_file <= ir.params.ldr_params.reg_b;
        oe_b_reg_file <= 1;
        mem_rd <= 1;
        sel_in_reg_file <= ir.params.ldr_params.reg_a;
        ld_reg_file <= 1;
      end

      // reg_a <- immediate
      LDI: begin
        oe_a_ir <= 1;
        b_reg_mask <= 32'hff;
        alu_op <= alu_pkg::PASSA;
        sel_in_reg_file <= ir.params.ld_params.reg_a;
        ld_reg_file <= 1;
      end

      // *address <- reg_a
      ST: begin
        sel_a_reg_file <= ir.params.st_params.reg_a;
        oe_a_reg_file <= 1;
        oe_b_ir <= 1;
        b_reg_mask <= 32'hff;
        mem_wr <= 1;
      end

	  // *reg_b <- reg_a
	  STR: begin
        sel_a_reg_file <= ir.params.str_params.reg_a;
        oe_a_reg_file <= 1;
        sel_b_reg_file <= ir.params.str_params.reg_b;
        oe_b_reg_file <= 1;
		mem_wr <= 1;
	  end

      STOP: ;
      default: ;
    endcase
  end

  task static do_binary_operation(input alu_op_e op, input logic set_status, input reg_e reg_a,
                                  input reg_e reg_b, input reg_e reg_c);
    begin
      sel_a_reg_file <= reg_b;
      oe_a_reg_file <= 1;
      sel_b_reg_file <= reg_c;
      oe_b_reg_file <= 1;
      alu_op <= op;
      sel_in_reg_file <= reg_a;
      ld_reg_file <= 1;
      ld_status <= set_status;
    end
  endtask

  task static do_binary_operation_i(input alu_op_e op, input logic set_status, input reg_e reg_a,
                                    input reg_e reg_b);
    begin
      sel_a_reg_file <= reg_b;
      oe_a_reg_file <= 1;
      oe_b_ir <= 1;
      b_reg_mask <= 32'hf;
      alu_op <= op;
      sel_in_reg_file <= reg_a;
      ld_reg_file <= 1;
      ld_status <= set_status;
    end
  endtask

  task static do_reverse_binary_operation_i(input alu_op_e op, input logic set_status,
                                            input reg_e reg_a, input reg_e reg_b);
    begin
      oe_a_ir <= 1;
      a_reg_mask <= 32'hf;
      sel_b_reg_file <= reg_b;
      oe_b_reg_file <= 1;
      alu_op <= op;
      sel_in_reg_file <= reg_a;
      ld_reg_file <= 1;
      ld_status <= set_status;
    end
  endtask

  task static do_unary_operation(input alu_op_e op, input logic set_status, input reg_e reg_a,
                                 input reg_e reg_b);
    begin
      sel_a_reg_file <= reg_b;
      oe_a_reg_file <= 1;
      alu_op <= op;
      sel_in_reg_file <= reg_a;
      ld_reg_file <= 1;
      ld_status <= set_status;
    end
  endtask

  task static do_unary_operation_i(input alu_op_e op, input logic set_status, input reg_e reg_a);
    begin
      oe_a_ir <= 1;
      a_reg_mask <= 32'hf;
      alu_op <= op;
      sel_in_reg_file <= reg_a;
      ld_reg_file <= 1;
      ld_status <= set_status;
    end
  endtask
endmodule
