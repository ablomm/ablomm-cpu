import cu_pkg::*;
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
    output logic signed [11:0] b_reg_offset,

    output reg_e sel_a_reg,
    output reg_e sel_b_reg,
    output reg_e sel_in_reg,

    output logic oe_a_reg,
    output logic oe_b_reg,
    output logic ld_reg,

    output logic ld_pc_lr,

    output logic post_inc_sp,
    output logic pre_dec_sp,
    output logic post_inc_pc,

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

    EXCEPT1,
    EXCEPT2,

    FETCH,
	NOP,
    NOT,
    LD,
    LDR,
    LDI,
    ST,
    STR,
    PUSH,
    POP,
    ALU
  } states_e;

  states_e state = STOP;

  always @(posedge rst) state <= STOP;

  // state changes
  // negative edge to fix race conditions
  always_ff @(negedge clk) begin
    unique case (state)
      FETCH: begin
        if (hwint & status.imask) state <= HWINT1;
        else if (satisfies_condition(ir.condition, status.alu_status)) begin
          unique casez (ir.instruction)
            cu_pkg::NOP: state <= NOP;
            cu_pkg::LD: begin
              if (ir.operands.ld.reg_a === reg_pkg::STATUS && status.mode !== reg_pkg::SUPERVISOR)
                state <= EXCEPT1;
              else state <= LD;
            end
            cu_pkg::LDR: begin
              if (ir.operands.ldr.reg_a === reg_pkg::STATUS && status.mode !== reg_pkg::SUPERVISOR)
                state <= EXCEPT1;
              else state <= LDR;
            end
            cu_pkg::LDI: begin
              if (ir.operands.ldi.reg_a === reg_pkg::STATUS && status.mode !== reg_pkg::SUPERVISOR)
                state <= EXCEPT1;
              else state <= LDI;
            end
            cu_pkg::ST: state <= ST;
            cu_pkg::STR: state <= STR;
            cu_pkg::PUSH: state <= PUSH;
            cu_pkg::POP: begin
              if (ir.operands.pop.reg_a === reg_pkg::STATUS && status.mode !== reg_pkg::SUPERVISOR)
                state <= EXCEPT1;
              else state <= POP;
            end
            cu_pkg::INT: state <= SWINT1;
            // all alu ops will have a f in first instruction nibble
            // the second nibble will be the alu_op
            'hf?: begin
              // exception if try to load status when not in supervisor mode
              if (ir.operands.unknown_alu_op.reg_a === reg_pkg::STATUS && status.mode !== reg_pkg::SUPERVISOR)
                state <= EXCEPT1;
              else state <= ALU;
            end
            default: state <= EXCEPT1;
          endcase
        end
      end

      SWINT1: state <= SWINT2;
      HWINT1: state <= HWINT2;
      EXCEPT1: state <= EXCEPT2;

      STOP: if (start) state <= FETCH;
      default: state <= FETCH;
    endcase
  end

  function static logic satisfies_condition(input cond_e condition, input alu_status_t status);
    begin
      unique case (condition)
        cu_pkg::NONE: satisfies_condition = 1;
        cu_pkg::EQ: satisfies_condition = status.zero;
        cu_pkg::NE: satisfies_condition = !status.zero;
        cu_pkg::ULT: satisfies_condition = !status.carry;
        cu_pkg::UGT: satisfies_condition = status.carry && !status.zero;
        cu_pkg::ULE: satisfies_condition = !status.carry || status.zero;
        cu_pkg::UGE: satisfies_condition = status.carry;
        cu_pkg::SLT: satisfies_condition = status.negative !== status.overflow;
        cu_pkg::SGT: satisfies_condition = !status.zero && (status.negative === status.overflow);
        cu_pkg::SLE: satisfies_condition = status.zero || (status.negative !== status.overflow);
        cu_pkg::SGE: satisfies_condition = status.negative === status.overflow;
        default: satisfies_condition = 1;
      endcase
    end

  endfunction

  // outputs
  always_comb begin
    //defaults
    mem_rd = 0;
    mem_wr = 0;

    oe_alu = 0;
    alu_op = alu_op_e'(0);

    sel_a_reg = reg_e'(0);
    sel_b_reg = reg_e'(0);
    sel_in_reg = reg_e'(0);

    oe_a_reg = 0;
    oe_b_reg = 0;
    ld_reg = 0;

    ld_pc_lr = 0;

    post_inc_sp = 0;
    pre_dec_sp = 0;
    post_inc_pc = 0;

    oe_a_consts = 0;
    oe_b_consts = 0;

    oe_a_ir = 0;
    oe_b_ir = 0;
    ld_ir = 0;

    ld_alu_status = 0;
    imask_in = 0;
    ld_imask = 0;
    mode_in = cpu_mode_e'(0);
    ld_mode = 0;

    a_reg_mask = 32'hffffffff;
    b_reg_mask = 32'hffffffff;
    b_reg_offset = 0;

    unique case (state)

      // ir <- *(pc++)
      FETCH: begin
        sel_b_reg = reg_pkg::PC;
        oe_b_reg = 1;
        mem_rd = 1;
        ld_ir = 1;
        post_inc_pc = 1;
      end

      NOP: ;

      // reg_a <- *address
      LD: begin
        oe_b_ir = 1;
        b_reg_mask = 32'hffff;
        mem_rd = 1;
        sel_in_reg = reg_e'(ir.operands.ld.reg_a);
        ld_reg = 1;

        ld_pc_lr = ir.operands.ld.reg_a == reg_pkg::PCL;
      end

      // reg_a <- *reg_b
      LDR: begin
        sel_b_reg = reg_e'(ir.operands.ldr.reg_b);
        oe_b_reg = 1;
        b_reg_offset = ir.operands.ldr.offset;
        mem_rd = 1;
        sel_in_reg = reg_e'(ir.operands.ldr.reg_a);
        ld_reg = 1;

        ld_pc_lr = ir.operands.ldr.reg_a == reg_pkg::PCL;
      end

      // reg_a <- immediate
      LDI: begin
        oe_b_ir = 1;
        b_reg_mask = 32'hffff;
        alu_op = alu_pkg::PASS;
        oe_alu = 1;
        sel_in_reg = reg_e'(ir.operands.ld.reg_a);
        ld_reg = 1;

        ld_pc_lr = ir.operands.ldi.reg_a == reg_pkg::PCL;
      end

      // *address <- reg_a
      ST: begin
        sel_a_reg = reg_e'(ir.operands.st.reg_a);
        oe_a_reg = 1;
        oe_b_ir = 1;
        b_reg_mask = 32'hffff;
        mem_wr = 1;
      end

      // *reg_b <- reg_a
      STR: begin
        sel_a_reg = reg_e'(ir.operands.str.reg_a);
        oe_a_reg = 1;
        sel_b_reg = reg_e'(ir.operands.str.reg_b);
        oe_b_reg = 1;
        b_reg_offset = ir.operands.str.offset;
        mem_wr = 1;
      end

      // *(--sp) <- reg_a
      PUSH: begin
        pre_dec_sp = 1;
        sel_a_reg = reg_e'(ir.operands.push.reg_a);
        oe_a_reg = 1;
        sel_b_reg = reg_pkg::SP;
        oe_b_reg = 1;
        mem_wr = 1;
      end

      // reg_a <- *(sp++)
      POP: begin
        sel_b_reg = reg_pkg::SP;
        oe_b_reg = 1;
        mem_rd = 1;
        sel_in_reg = reg_e'(ir.operands.pop.reg_a);
        ld_reg = 1;
        post_inc_sp = 1;

        ld_pc_lr = ir.operands.pop.reg_a == reg_pkg::PCL;
      end

      ALU: begin
        if (ir.operands.unknown_alu_op.flags.immediate) begin
          if (ir.operands.alu_op_i.flags.reverse) begin
            oe_a_ir = 1;
            a_reg_mask = 32'hff;
            sel_b_reg = reg_e'(ir.operands.alu_op_i.reg_b);
            oe_b_reg = 1;
          end else begin
            sel_a_reg = reg_e'(ir.operands.alu_op_i.reg_b);
            oe_a_reg = 1;
            oe_b_ir = 1;
            b_reg_mask = 32'hff;
          end

        end else begin
          if (ir.operands.alu_op.flags.reverse) begin
            sel_a_reg = reg_e'(ir.operands.alu_op.reg_c);
            sel_b_reg = reg_e'(ir.operands.alu_op.reg_b);
          end else begin
            sel_a_reg = reg_e'(ir.operands.alu_op.reg_b);
            sel_b_reg = reg_e'(ir.operands.alu_op.reg_c);
          end

          oe_a_reg = 1;
          oe_b_reg = 1;
        end

        alu_op = alu_op_e'(ir[23:20]);  // the alu op will always be the second nibble of the instruction
        oe_alu = ~ir.operands.unknown_alu_op.flags.loadn;
        sel_in_reg = reg_e'(ir.operands.unknown_alu_op.reg_a);
        ld_reg = ~ir.operands.unknown_alu_op.flags.loadn;
        ld_alu_status = ir.operands.unknown_alu_op.flags.set_status;

        ld_pc_lr = ir.operands.unknown_alu_op.reg_a == reg_pkg::PCL;
      end

      // push PC
      SWINT1, HWINT1, EXCEPT1: begin
        pre_dec_sp = 1;
        sel_a_reg = reg_pkg::PC;
        oe_a_reg = 1;
        sel_b_reg = reg_pkg::SP;
        oe_b_reg = 1;
        mem_wr = 1;
      end

      // PC <- 00000001
      // imask <- 0
      // mode <- SUPERVISOR
      HWINT2: begin
        sel_b_reg = reg_e'(4'h1);
        oe_b_consts = 1;
        alu_op = alu_pkg::PASS;
        oe_alu = 1;
        sel_in_reg = reg_pkg::PC;
        ld_reg = 1;
        imask_in = 0;
        ld_imask = 1;
        mode_in = reg_pkg::SUPERVISOR;
        ld_mode = 1;
      end

      // PC <- 00000002
      // mode <- SUPERVISOR
      SWINT2: begin
        sel_b_reg = reg_e'(4'h2);
        oe_b_consts = 1;
        alu_op = alu_pkg::PASS;
        oe_alu = 1;
        sel_in_reg = reg_pkg::PC;
        ld_reg = 1;
        mode_in = reg_pkg::SUPERVISOR;
        ld_mode = 1;
      end

      // pc <- 00000003
      // mode <- SUPERVISOR
      EXCEPT2: begin
        sel_b_reg = reg_e'(4'h3);
        oe_b_consts = 1;
        alu_op = alu_pkg::PASS;
        oe_alu = 1;
        sel_in_reg = reg_pkg::PC;
        ld_reg = 1;
        mode_in = reg_pkg::SUPERVISOR;
        ld_mode = 1;
      end
      default: ;
    endcase
  end
endmodule
