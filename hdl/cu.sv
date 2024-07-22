import cpu_pkg::*;
import alu_pkg::*;
import reg_pkg::*;

module cu (
    input clk,
    input wire start,
    input wire rst,
    input wire hwint,

    input wire ir_t ir,
    input wire status_t status,

    output logic mem_rd,
    output logic mem_wr,

    output logic oe_alu,
    output alu_op_e alu_op,

    output logic [31:0] a_reg_mask,
    output logic [31:0] b_reg_mask,

    output reg_e sel_a_reg,
    output reg_e sel_b_reg,
    output reg_e sel_in_reg,

    output logic oe_a_reg_file,
    output logic oe_b_reg_file,
    output logic ld_reg_file,
    output logic sp_post_inc,
    output logic sp_pre_dec,
    output logic pc_post_inc,

    output logic oe_a_consts,
    output logic oe_b_consts,

    output logic oe_a_ir,
    output logic oe_b_ir,
    output logic ld_ir,

    output logic ld_alu_status,
    output logic imask_in,
    output logic ld_imask,
    output cpu_mode_e mode_in,
    output logic ld_mode
);

  // internal states of the CU, one CPU instruction could have many internal
  // CU states (multi-clock instructions)
  typedef enum {
    STOP,
    HWINT1,
    HWINT2,
    SWINT1,
    SWINT2,
    CLRI,
    SETI,
    FETCH,
    NOP,
    ALU,
    LD,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP
  } states_e;

  states_e state = STOP;

  always @(posedge rst) state <= STOP;

  // state changes
  always_ff @(posedge clk) begin
    case (state)
      FETCH: begin
        if (hwint & status.imask) state <= HWINT1;
        else if (satisfies_condition(ir.condition, status.alu_status)) begin
          casez (ir.instruction)
            // all alu ops will have a 0 in first instruction nibble
            // the second nibble will be the alu_op
            'h0?: state <= ALU;
            cpu_pkg::LD: state <= LD;
            cpu_pkg::LDR: state <= LDR;
            cpu_pkg::LDI: state <= LDI;
            cpu_pkg::ST: state <= ST;
            cpu_pkg::STR: state <= STR;
            cpu_pkg::PUSH: state <= PUSH;
            cpu_pkg::POP: state <= POP;
            cpu_pkg::INT: state <= SWINT1;
            cpu_pkg::CLRI: state <= CLRI;
            cpu_pkg::SETI: state <= SETI;
            default: state <= NOP;
          endcase
        end
      end
      SWINT1: state <= SWINT2;
      HWINT1: state <= HWINT2;
      STOP: if (start) state <= FETCH;
      default: state <= FETCH;
    endcase
  end

  function static logic satisfies_condition(input cond_e condition, input alu_status_t status);
    begin
      unique case (condition)
        cpu_pkg::NONE: satisfies_condition = 1;
        cpu_pkg::EQ: satisfies_condition = status.zero;
        cpu_pkg::NE: satisfies_condition = !status.zero;
        cpu_pkg::LTU: satisfies_condition = !status.carry;
        cpu_pkg::GTU: satisfies_condition = status.carry && !status.zero;
        cpu_pkg::LEU: satisfies_condition = !status.carry || status.zero;
        cpu_pkg::GEU: satisfies_condition = status.carry;
        cpu_pkg::LTS: satisfies_condition = status.negative !== status.overflow;
        cpu_pkg::GTS: satisfies_condition = !status.zero && (status.negative === status.overflow);
        cpu_pkg::LES: satisfies_condition = status.zero || (status.negative !== status.overflow);
        cpu_pkg::GES: satisfies_condition = status.negative === status.overflow;
        default: satisfies_condition = 1;
      endcase
    end

  endfunction

  // outputs
  always @(state) begin
    {
      mem_rd,
      mem_wr,

      oe_alu,
      alu_op,

      sel_a_reg,
      sel_b_reg,
      sel_in_reg,

      oe_a_reg_file,
      oe_b_reg_file,
      ld_reg_file,
      sp_post_inc,
      sp_pre_dec,
      pc_post_inc,

      oe_a_consts,
      oe_b_consts,

      oe_a_ir,
      oe_b_ir,
      ld_ir,

      ld_alu_status,
      imask_in,
      ld_imask,
      mode_in,
      ld_mode
    } <= 0;

    a_reg_mask <= 32'hffffffff;
    b_reg_mask <= 32'hffffffff;

    case (state)

      // ir <- *(pc++)
      FETCH: begin
        sel_b_reg <= reg_pkg::PC;
        oe_b_reg_file <= 1;
        mem_rd <= 1;
        ld_ir <= 1;
        pc_post_inc <= 1;
      end


      ALU: begin
        if (ir.params.unknown_alu_op.flags.immediate) begin
          if (ir.params.alu_op_i.flags.reverse) begin
            oe_a_ir <= 1;
            a_reg_mask <= 32'hff;
            sel_b_reg <= ir.params.alu_op_i.reg_b;
            oe_b_reg_file <= 1;
          end else begin
            sel_a_reg <= ir.params.alu_op_i.reg_b;
            oe_a_reg_file <= 1;
            oe_b_ir <= 1;
            b_reg_mask <= 32'hff;
          end

        end else begin
          if (ir.params.alu_op.flags.reverse) begin
            sel_a_reg <= ir.params.alu_op.reg_c;
            sel_b_reg <= ir.params.alu_op.reg_b;
          end else begin
            sel_a_reg <= ir.params.alu_op.reg_b;
            sel_b_reg <= ir.params.alu_op.reg_c;
          end

          oe_a_reg_file <= 1;
          oe_b_reg_file <= 1;
        end

        alu_op <= ir[23:20];  // the alu op will always be the second nibble of the instruction
        oe_alu <= ir.params.alu_op_i.flags.load;
        sel_in_reg <= ir.params.alu_op_i.reg_a;
        ld_reg_file <= ir.params.alu_op_i.flags.load;
        ld_alu_status <= ir.params.alu_op_i.flags.set_status;
      end

      // reg_a <- *address
      LD: begin
        oe_b_ir <= 1;
        b_reg_mask <= 32'hffff;
        mem_rd <= 1;
        sel_in_reg <= ir.params.ld_params.reg_a;
        ld_reg_file <= 1;
      end

      // reg_a <- *reg_b
      LDR: begin
        sel_b_reg <= ir.params.ldr_params.reg_b;
        oe_b_reg_file <= 1;
        mem_rd <= 1;
        sel_in_reg <= ir.params.ldr_params.reg_a;
        ld_reg_file <= 1;
      end

      // reg_a <- immediate
      LDI: begin
        oe_a_ir <= 1;
        a_reg_mask <= 32'hffff;
        alu_op <= alu_pkg::PASSA;
        oe_alu <= 1;
        sel_in_reg <= ir.params.ld_params.reg_a;
        ld_reg_file <= 1;
      end

      // *address <- reg_a
      ST: begin
        sel_a_reg <= ir.params.st_params.reg_a;
        oe_a_reg_file <= 1;
        oe_b_ir <= 1;
        b_reg_mask <= 32'hffff;
        mem_wr <= 1;
      end

      // *reg_b <- reg_a
      STR: begin
        sel_a_reg <= ir.params.str_params.reg_a;
        oe_a_reg_file <= 1;
        sel_b_reg <= ir.params.str_params.reg_b;
        oe_b_reg_file <= 1;
        mem_wr <= 1;
      end

      // *(--sp) <- reg_a
      PUSH: begin
        sel_a_reg <= ir.params.push_params.reg_a;
        sel_b_reg <= reg_pkg::SP;
        sp_pre_dec <= 1;
        mem_wr <= 1;
      end

      // reg_a <- *(sp++)
      POP: begin
        sel_b_reg <= reg_pkg::SP;
        sp_post_inc <= 1;
        mem_rd <= 1;
        sel_in_reg <= ir.params.pop_params.reg_a;
        ld_reg_file <= 1;
      end

      // push PC
      // imask <- 0
      SWINT1, HWINT1: begin
        sel_a_reg <= reg_pkg::PC;
        sel_b_reg <= reg_pkg::SP;
        sp_pre_dec <= 1;
        mem_wr <= 1;
        imask_in <= 0;
        ld_imask <= 1;
      end

      // PC <- 00000001
      HWINT2: begin
        sel_a_reg <= 4'h1;
        oe_a_consts <= 1;
        alu_op <= alu_pkg::PASSA;
        oe_alu <= 1;
        sel_in_reg <= reg_pkg::PC;
        ld_reg_file <= 1;
      end

      // PC <- 00000002
      SWINT2: begin
        sel_a_reg <= 4'h2;
        oe_a_consts <= 1;
        alu_op <= alu_pkg::PASSA;
        oe_alu <= 1;
        sel_in_reg <= reg_pkg::PC;
        ld_reg_file <= 1;
      end

      CLRI: begin
        imask_in <= 0;
        ld_imask <= 1;
      end

      SETI: begin
        imask_in <= 1;
        ld_imask <= 1;
      end

      default: ;
    endcase
  end
endmodule
