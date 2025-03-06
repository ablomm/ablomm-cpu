import reg_pkg::*;

module status_reg_tb;
  logic clk;
  logic rst = 0;
  tri [5:0] a, b;
  logic [5:0] in;
  logic oe_a, oe_b, ld;
  alu_status_t alu_status_in;
  logic ld_alu_status;
  logic imask_in;
  logic ld_imask;
  cpu_mode_e mode_in;
  logic ld_mode;
  status_t value;

  status_reg status (.*);

  initial begin
    #1200;
    $display("\ntesting status_reg");
    test_ld_oe(status_t'('b101011));
    test_ld_oe(status_t'('b010100));
    test_ld_alu_status(alu_status_t'('b1111));
    test_ld_alu_status(alu_status_t'('b1010));
    test_ld_imask('b1);
    test_ld_imask('b0);
    test_ld_mode(reg_pkg::SUPERVISOR);
    test_ld_mode(reg_pkg::USER);
    test_ld_oe(status_t'('b111111));
    test_ld_user_oe(status_t'('b101001));

  end

  task static load(input status_t data_in);
    begin
      clk  = 0;
      oe_a = 0;
      oe_b = 0;
      #1;

      in  = data_in;
      ld  = 1;
      clk = 1;
      #1;

      ld = 0;
      #1;
    end
  endtask

  task static get_from_a(output logic [5:0] value_out);
    begin
      oe_a = 1;
      #1;

      value_out = value;
      oe_a = 0;
      #1;
    end
  endtask

  task static get_from_b(output logic [5:0] value_out);
    begin
      oe_b = 1;
      #1;

      value_out = value;
      oe_b = 0;
      #1;
    end
  endtask

  task static test_ld_oe(input status_t data_in);
    begin
      status_t read_value;

      test_ld_mode(reg_pkg::SUPERVISOR);

      load(data_in);

      get_from_a(read_value);
      $display("ld oe a: a = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;

      get_from_b(read_value);
      $display("ld oe b: b = %d, expected = %d", read_value, data_in);
      assert (read_value === data_in)
      else $fatal;
    end
  endtask

  task static test_ld_user_oe(input status_t data_in);
    begin
      status_t read_value;
      status_t value_before;
      value_before = value;

      test_ld_mode(reg_pkg::USER);

      // change the imask to test
      data_in.imask = ~value_before.imask;

      load(data_in);

      get_from_a(read_value);
      $display("ld user oe a: a = %d, data_in = %d", read_value, data_in);
      assert (read_value.alu_status === data_in.alu_status)
      else $fatal;

      assert (read_value.imask !== data_in.imask && read_value.imask === value_before.imask)
      else $fatal;

      assert (read_value.mode === reg_pkg::USER)
      else $fatal;

      get_from_b(read_value);
      $display("ld user oe b: b = %d, data_in = %d", read_value, data_in);
      assert (read_value.alu_status === data_in.alu_status)
      else $fatal;

      assert (read_value.imask !== data_in.imask && read_value.imask === value_before.imask)
      else $fatal;

      assert (read_value.mode === reg_pkg::USER)
      else $fatal;
    end
  endtask

  task static test_ld_alu_status(input alu_status_t alu_status);
    begin
      status_t value_before;
      value_before = value;

      clk = 0;
      alu_status_in = alu_status;
      ld_alu_status = 1;
      #1;

      clk = 1;
      #1;

      ld_alu_status = 0;
      #1;

      $display("ld alu status: out: %b, expected = %b", value.alu_status, alu_status);
      assert (value.alu_status === alu_status)
      else $fatal;

      // make sure it didn't change anything else
      assert (value_before.imask === value.imask)
      else $fatal;

      assert (value_before.mode === value.mode)
      else $fatal;
    end
  endtask

  task static test_ld_imask(input logic imask);
    begin
      status_t value_before;
      value_before = value;

      clk = 0;
      imask_in = imask;
      ld_imask = 1;
      #1;

      clk = 1;
      #1;

      ld_imask = 0;
      #1;

      $display("ld imask: out: %b, expected = %b", value.imask, imask);
      assert (value.imask === imask)
      else $fatal;

      // make sure it didn't change anything else
      assert (value_before.alu_status === value.alu_status)
      else $fatal;

      assert (value_before.mode === value.mode)
      else $fatal;
    end
  endtask

  task static test_ld_mode(input cpu_mode_e mode);
    begin
      status_t value_before;
      value_before = value;

      clk = 0;
      mode_in = mode;
      ld_mode = 1;
      #1;

      clk = 1;
      #1;

      ld_mode = 0;
      #1;

      $display("ld mode: out: %b, expected = %b", value.mode, mode);
      assert (value.mode === mode)
      else $fatal;

      // make sure it didn't change anything else
      assert (value_before.alu_status === value.alu_status)
      else $fatal;

      assert (value_before.imask === value.imask)
      else $fatal;
    end
  endtask
endmodule
