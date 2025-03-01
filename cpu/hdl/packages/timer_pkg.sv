package timer_pkg;
  typedef struct packed {
    // underscores because these are keywords in sv
    logic _continue;
    logic _start;
  } timer_ctrl_t;

  typedef enum logic [1:0] {
    ACK = 2'h0,
    CTRL,
    INTERVAL,
    TIMER
  } timer_reg_e;

endpackage
