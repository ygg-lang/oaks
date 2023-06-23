// Verilog Test File - Comprehensive Syntax Coverage
// This file tests various Verilog syntax elements for lexer testing

// Comments
// Single line comment
/* Multi-line comment
   can span multiple lines
   and contain any text */

// Compiler directives
`timescale 1ns / 1ps
`define WORD_SIZE 32
`define MAX_COUNT 1024
`define CLOCK_PERIOD 10
`ifdef SIMULATION
    `define DEBUG
`endif

`include "memory.v"
`include "alu.v"

// Module declarations
module basic_test;

    // Parameter declarations
    parameter DATA_WIDTH = 32;
    parameter ADDR_WIDTH = 16;
    parameter MEMORY_SIZE = 1024;
    parameter CLOCK_FREQ = 100_000_000; // 100 MHz
    
    // Localparam declarations
    localparam STATE_IDLE = 2'b00;
    localparam STATE_READ = 2'b01;
    localparam STATE_WRITE = 2'b10;
    localparam STATE_DONE = 2'b11;

    // Wire declarations
    wire clk;
    wire reset;
    wire enable;
    wire [DATA_WIDTH-1:0] data_in;
    wire [DATA_WIDTH-1:0] data_out;
    wire [ADDR_WIDTH-1:0] address;
    wire read_enable;
    wire write_enable;
    wire chip_select;
    wire output_enable;
    wire ready;
    wire error;

    // Reg declarations
    reg clk_reg;
    reg reset_reg;
    reg [DATA_WIDTH-1:0] data_reg;
    reg [ADDR_WIDTH-1:0] addr_reg;
    reg [7:0] counter;
    reg [1:0] state;
    reg [1:0] next_state;
    reg valid;
    reg busy;

    // Integer declarations
    integer i, j, k;
    integer file_handle;
    integer error_count;
    integer cycle_count;

    // Real declarations
    real frequency;
    real period;
    real duty_cycle;

    // Time declarations
    time current_time;
    time start_time;
    time end_time;

    // Memory declarations
    reg [7:0] memory [0:MEMORY_SIZE-1];
    reg [DATA_WIDTH-1:0] register_file [0:31];
    reg [15:0] instruction_memory [0:1023];

    // Array of instances
    wire [7:0] bus_data [0:3];
    reg [3:0] control_signals [0:7];

    // Number formats
    // Binary numbers
    reg [7:0] binary_val = 8'b10101010;
    reg [15:0] binary_val2 = 16'b1111_0000_1010_0101;
    
    // Octal numbers
    reg [11:0] octal_val = 12'o7654;
    reg [8:0] octal_val2 = 9'o377;
    
    // Decimal numbers
    reg [7:0] decimal_val = 8'd255;
    reg [15:0] decimal_val2 = 16'd65535;
    
    // Hexadecimal numbers
    reg [7:0] hex_val = 8'hFF;
    reg [31:0] hex_val2 = 32'hDEAD_BEEF;
    reg [15:0] hex_val3 = 16'hA5A5;

    // Sized and unsized numbers
    reg [7:0] sized_num = 8'b10101010;
    reg [31:0] unsized_num = 'hFFFFFFFF;
    reg [3:0] x_z_values = 4'b10xz;

    // String literals
    reg [8*20:1] string_val = "Hello, Verilog World";
    reg [8*10:1] short_string = "Test";

    // Assign statements
    assign data_out = enable ? data_reg : {DATA_WIDTH{1'bz}};
    assign ready = ~busy & valid;
    assign error = (state == STATE_DONE) & ~valid;
    assign address = addr_reg;

    // Continuous assignments with delays
    assign #5 clk = clk_reg;
    assign #(2,3) reset = reset_reg;

    // Conditional assignments
    assign data_out = (chip_select && output_enable) ? 
                      memory[address[9:0]] : 8'bz;

    // Bitwise operations
    wire [7:0] and_result = data_in[7:0] & 8'hFF;
    wire [7:0] or_result = data_in[7:0] | 8'h00;
    wire [7:0] xor_result = data_in[7:0] ^ 8'hAA;
    wire [7:0] not_result = ~data_in[7:0];
    wire [7:0] nand_result = ~(data_in[7:0] & 8'hFF);
    wire [7:0] nor_result = ~(data_in[7:0] | 8'h00);
    wire [7:0] xnor_result = ~(data_in[7:0] ^ 8'hAA);

    // Reduction operations
    wire and_reduce = &data_in[7:0];
    wire or_reduce = |data_in[7:0];
    wire xor_reduce = ^data_in[7:0];
    wire nand_reduce = ~&data_in[7:0];
    wire nor_reduce = ~|data_in[7:0];
    wire xnor_reduce = ~^data_in[7:0];

    // Arithmetic operations
    wire [8:0] add_result = data_in[7:0] + 8'd1;
    wire [7:0] sub_result = data_in[7:0] - 8'd1;
    wire [15:0] mul_result = data_in[7:0] * 8'd3;
    wire [7:0] div_result = data_in[7:0] / 8'd2;
    wire [7:0] mod_result = data_in[7:0] % 8'd3;
    wire [15:0] power_result = data_in[7:0] ** 2;

    // Shift operations
    wire [7:0] left_shift = data_in[7:0] << 2;
    wire [7:0] right_shift = data_in[7:0] >> 2;
    wire [7:0] arith_left_shift = data_in[7:0] <<< 2;
    wire [7:0] arith_right_shift = data_in[7:0] >>> 2;

    // Relational operations
    wire greater = data_in[7:0] > 8'd128;
    wire less = data_in[7:0] < 8'd128;
    wire greater_equal = data_in[7:0] >= 8'd128;
    wire less_equal = data_in[7:0] <= 8'd128;

    // Equality operations
    wire logical_equal = data_in[7:0] == 8'd128;
    wire logical_not_equal = data_in[7:0] != 8'd128;
    wire case_equal = data_in[7:0] === 8'd128;
    wire case_not_equal = data_in[7:0] !== 8'd128;

    // Logical operations
    wire logical_and = (data_in[7:0] > 0) && (data_in[15:8] > 0);
    wire logical_or = (data_in[7:0] > 0) || (data_in[15:8] > 0);
    wire logical_not = !(data_in[7:0] > 0);

    // Conditional operator
    wire [7:0] conditional_result = (enable) ? data_in[7:0] : 8'h00;
    wire [7:0] nested_conditional = (state == STATE_READ) ? data_in[7:0] :
                                   (state == STATE_WRITE) ? 8'hFF : 8'h00;

    // Concatenation and replication
    wire [15:0] concat_result = {data_in[7:0], 8'h00};
    wire [31:0] replicate_result = {4{data_in[7:0]}};
    wire [23:0] mixed_concat = {2'b10, data_in[7:0], 14'h3FFF};

    // Part select
    wire [3:0] upper_nibble = data_in[7:4];
    wire [3:0] lower_nibble = data_in[3:0];
    wire single_bit = data_in[0];

    // Indexed part select
    wire [3:0] indexed_select = data_in[counter[2:0] +: 4];
    wire [3:0] indexed_select2 = data_in[counter[2:0] -: 4];

    // Always blocks
    // Combinational logic
    always @(*) begin
        case (state)
            STATE_IDLE: begin
                next_state = enable ? STATE_READ : STATE_IDLE;
                busy = 1'b0;
                valid = 1'b0;
            end
            STATE_READ: begin
                next_state = STATE_WRITE;
                busy = 1'b1;
                valid = 1'b0;
            end
            STATE_WRITE: begin
                next_state = STATE_DONE;
                busy = 1'b1;
                valid = 1'b0;
            end
            STATE_DONE: begin
                next_state = STATE_IDLE;
                busy = 1'b0;
                valid = 1'b1;
            end
            default: begin
                next_state = STATE_IDLE;
                busy = 1'b0;
                valid = 1'b0;
            end
        endcase
    end

    // Sequential logic
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            state <= STATE_IDLE;
            counter <= 8'h00;
            data_reg <= {DATA_WIDTH{1'b0}};
            addr_reg <= {ADDR_WIDTH{1'b0}};
        end else begin
            state <= next_state;
            counter <= counter + 1'b1;
            
            if (enable) begin
                data_reg <= data_in;
                addr_reg <= address;
            end
        end
    end

    // Clock generation
    always begin
        clk_reg = 1'b0;
        #(`CLOCK_PERIOD/2);
        clk_reg = 1'b1;
        #(`CLOCK_PERIOD/2);
    end

    // Memory operations
    always @(posedge clk) begin
        if (write_enable && chip_select) begin
            memory[address[9:0]] <= data_in[7:0];
        end
    end

    // Case statements
    always @(*) begin
        case (data_in[1:0])
            2'b00: data_reg[7:0] = 8'h00;
            2'b01: data_reg[7:0] = 8'h55;
            2'b10: data_reg[7:0] = 8'hAA;
            2'b11: data_reg[7:0] = 8'hFF;
        endcase
    end

    // Casex and casez statements
    always @(*) begin
        casex (data_in[3:0])
            4'b000x: addr_reg[3:0] = 4'h0;
            4'b001x: addr_reg[3:0] = 4'h1;
            4'b01xx: addr_reg[3:0] = 4'h2;
            4'b1xxx: addr_reg[3:0] = 4'h3;
            default: addr_reg[3:0] = 4'hF;
        endcase
    end

    always @(*) begin
        casez (data_in[7:4])
            4'b000?: counter[3:0] = 4'h0;
            4'b001?: counter[3:0] = 4'h1;
            4'b01??: counter[3:0] = 4'h2;
            4'b1???: counter[3:0] = 4'h3;
            default: counter[3:0] = 4'hF;
        endcase
    end

    // If-else statements
    always @(*) begin
        if (reset) begin
            error_count = 0;
        end else if (error) begin
            error_count = error_count + 1;
        end else if (valid) begin
            error_count = error_count;
        end else begin
            error_count = 0;
        end
    end

    // For loops
    always @(*) begin
        for (i = 0; i < 32; i = i + 1) begin
            register_file[i] = {DATA_WIDTH{1'b0}};
        end
    end

    // While loops
    always @(*) begin
        j = 0;
        while (j < MEMORY_SIZE) begin
            memory[j] = 8'h00;
            j = j + 1;
        end
    end

    // Repeat loops
    always @(posedge clk) begin
        repeat (8) begin
            data_reg <= {data_reg[DATA_WIDTH-2:0], 1'b0};
            #1;
        end
    end

    // Forever loops (for testbenches)
    initial begin
        forever begin
            #100;
            cycle_count = cycle_count + 1;
        end
    end

    // Fork-join blocks
    initial begin
        fork
            begin
                #10 reset_reg = 1'b1;
                #20 reset_reg = 1'b0;
            end
            begin
                #5 enable = 1'b1;
                #50 enable = 1'b0;
            end
        join
    end

    // Disable statements
    initial begin
        begin : timeout_block
            #1000;
            $display("Timeout occurred");
        end
    end

    always @(posedge ready) begin
        disable timeout_block;
    end

    // Wait statements
    initial begin
        wait (reset == 1'b0);
        wait (ready == 1'b1);
        $display("System ready");
    end

    // Event declarations and triggers
    event data_ready;
    event transfer_complete;

    always @(posedge clk) begin
        if (valid) begin
            -> data_ready;
        end
    end

    always @(data_ready) begin
        $display("Data ready event triggered at time %t", $time);
        -> transfer_complete;
    end

    // Named blocks
    always @(posedge clk) begin : sequential_block
        reg [7:0] temp_data;
        temp_data = data_in[7:0];
        data_reg <= temp_data;
    end

    // Generate statements
    genvar gen_i;
    generate
        for (gen_i = 0; gen_i < 8; gen_i = gen_i + 1) begin : gen_loop
            assign bus_data[gen_i] = data_in[gen_i*8 +: 8];
        end
    endgenerate

    generate
        if (DATA_WIDTH == 32) begin : gen_32bit
            reg [31:0] wide_register;
        end else begin : gen_other
            reg [15:0] narrow_register;
        end
    endgenerate

    // Function declarations
    function [7:0] reverse_bits;
        input [7:0] data;
        integer idx;
        begin
            for (idx = 0; idx < 8; idx = idx + 1) begin
                reverse_bits[idx] = data[7-idx];
            end
        end
    endfunction

    function automatic [31:0] factorial;
        input [7:0] n;
        begin
            if (n <= 1)
                factorial = 1;
            else
                factorial = n * factorial(n - 1);
        end
    endfunction

    // Task declarations
    task automatic write_memory;
        input [ADDR_WIDTH-1:0] addr;
        input [7:0] data;
        begin
            @(posedge clk);
            addr_reg <= addr;
            data_reg[7:0] <= data;
            write_enable <= 1'b1;
            @(posedge clk);
            write_enable <= 1'b0;
        end
    endtask

    task automatic read_memory;
        input [ADDR_WIDTH-1:0] addr;
        output [7:0] data;
        begin
            @(posedge clk);
            addr_reg <= addr;
            read_enable <= 1'b1;
            @(posedge clk);
            data = memory[addr[9:0]];
            read_enable <= 1'b0;
        end
    endtask

    // System tasks and functions
    initial begin
        $display("Starting Verilog test simulation");
        $display("DATA_WIDTH = %d, ADDR_WIDTH = %d", DATA_WIDTH, ADDR_WIDTH);
        
        // Time formatting
        $timeformat(-9, 2, " ns", 10);
        
        // Monitor signals
        $monitor("Time: %t, State: %b, Data: %h, Address: %h", 
                 $time, state, data_reg, addr_reg);
        
        // File operations
        file_handle = $fopen("simulation.log", "w");
        $fdisplay(file_handle, "Simulation started at time %t", $time);
        
        // Memory initialization
        $readmemh("memory_init.hex", memory);
        $readmemb("register_init.bin", register_file);
        
        // Random number generation
        $random(12345); // Seed
        for (i = 0; i < 10; i = i + 1) begin
            data_reg <= $random;
            #10;
        end
        
        // Finish simulation
        #10000;
        $fclose(file_handle);
        $finish;
    end

    // Specify blocks for timing
    specify
        specparam tpd_clk_to_q = 5;
        specparam tsu_data = 2;
        specparam th_data = 1;
        
        (clk => data_out) = tpd_clk_to_q;
        $setup(data_in, posedge clk, tsu_data);
        $hold(posedge clk, data_in, th_data);
        
        $width(posedge clk, 4);
        $period(posedge clk, 10);
    endspecify

    // Primitive instantiations
    and and_gate (and_out, data_in[0], data_in[1]);
    or or_gate (or_out, data_in[2], data_in[3]);
    not not_gate (not_out, data_in[4]);
    nand nand_gate (nand_out, data_in[5], data_in[6]);
    nor nor_gate (nor_out, data_in[7], enable);
    xor xor_gate (xor_out, data_in[0], data_in[7]);
    xnor xnor_gate (xnor_out, data_in[1], data_in[6]);
    
    buf buf_gate (buf_out, data_in[0]);
    bufif0 bufif0_gate (bufif0_out, data_in[1], enable);
    bufif1 bufif1_gate (bufif1_out, data_in[2], enable);
    notif0 notif0_gate (notif0_out, data_in[3], enable);
    notif1 notif1_gate (notif1_out, data_in[4], enable);

    // Tri-state gates
    pmos pmos_gate (pmos_out, data_in[0], enable);
    nmos nmos_gate (nmos_out, data_in[1], enable);
    cmos cmos_gate (cmos_out, data_in[2], enable, ~enable);
    
    // Pull gates
    pullup (pull_up_net);
    pulldown (pull_down_net);

    // Switch primitives
    tranif0 switch0 (net1, net2, control);
    tranif1 switch1 (net3, net4, control);
    rtranif0 rswitch0 (net5, net6, control);
    rtranif1 rswitch1 (net7, net8, control);

endmodule

// Module with ports
module memory_controller (
    input wire clk,
    input wire reset,
    input wire [31:0] address,
    input wire [31:0] write_data,
    input wire write_enable,
    input wire read_enable,
    output reg [31:0] read_data,
    output reg ready,
    output reg error
);

    // Internal signals
    reg [1:0] state;
    reg [31:0] internal_address;
    
    // Memory array
    reg [31:0] memory [0:1023];
    
    // State machine
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            state <= 2'b00;
            ready <= 1'b0;
            error <= 1'b0;
            read_data <= 32'h0;
        end else begin
            case (state)
                2'b00: begin // IDLE
                    if (read_enable || write_enable) begin
                        state <= 2'b01;
                        internal_address <= address;
                        ready <= 1'b0;
                    end else begin
                        ready <= 1'b1;
                    end
                end
                2'b01: begin // ACCESS
                    if (write_enable) begin
                        memory[internal_address[9:0]] <= write_data;
                    end else if (read_enable) begin
                        read_data <= memory[internal_address[9:0]];
                    end
                    state <= 2'b10;
                end
                2'b10: begin // COMPLETE
                    ready <= 1'b1;
                    state <= 2'b00;
                end
                default: begin
                    state <= 2'b00;
                    error <= 1'b1;
                end
            endcase
        end
    end

endmodule

// Parameterized module
module fifo #(
    parameter DATA_WIDTH = 8,
    parameter DEPTH = 16,
    parameter ADDR_WIDTH = $clog2(DEPTH)
) (
    input wire clk,
    input wire reset,
    input wire push,
    input wire pop,
    input wire [DATA_WIDTH-1:0] data_in,
    output reg [DATA_WIDTH-1:0] data_out,
    output wire full,
    output wire empty,
    output reg [ADDR_WIDTH:0] count
);

    // Memory array
    reg [DATA_WIDTH-1:0] memory [0:DEPTH-1];
    
    // Pointers
    reg [ADDR_WIDTH-1:0] write_ptr;
    reg [ADDR_WIDTH-1:0] read_ptr;
    
    // Status flags
    assign full = (count == DEPTH);
    assign empty = (count == 0);
    
    // FIFO operations
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            write_ptr <= 0;
            read_ptr <= 0;
            count <= 0;
            data_out <= 0;
        end else begin
            case ({push & ~full, pop & ~empty})
                2'b01: begin // Pop only
                    data_out <= memory[read_ptr];
                    read_ptr <= read_ptr + 1;
                    count <= count - 1;
                end
                2'b10: begin // Push only
                    memory[write_ptr] <= data_in;
                    write_ptr <= write_ptr + 1;
                    count <= count + 1;
                end
                2'b11: begin // Push and pop
                    memory[write_ptr] <= data_in;
                    data_out <= memory[read_ptr];
                    write_ptr <= write_ptr + 1;
                    read_ptr <= read_ptr + 1;
                    // count remains the same
                end
                default: begin
                    // No operation
                end
            endcase
        end
    end

endmodule

// Interface declaration (SystemVerilog style, but showing syntax)
interface bus_interface #(parameter WIDTH = 32);
    logic [WIDTH-1:0] data;
    logic valid;
    logic ready;
    
    modport master (
        output data, valid,
        input ready
    );
    
    modport slave (
        input data, valid,
        output ready
    );
endinterface

// Top-level testbench module
module testbench;

    // Test signals
    reg clk;
    reg reset;
    reg [31:0] test_data;
    wire [31:0] result;
    
    // Clock generation
    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end
    
    // Reset generation
    initial begin
        reset = 1;
        #100 reset = 0;
    end
    
    // Test stimulus
    initial begin
        $dumpfile("testbench.vcd");
        $dumpvars(0, testbench);
        
        wait (!reset);
        
        // Test sequence
        repeat (100) begin
            @(posedge clk);
            test_data <= $random;
        end
        
        #1000;
        $finish;
    end
    
    // Instantiate DUT
    basic_test dut (
        .clk(clk),
        .reset(reset),
        .data_in(test_data),
        .data_out(result)
    );

endmodule

// UDP (User Defined Primitive)
primitive mux2to1 (out, sel, a, b);
    output out;
    input sel, a, b;
    
    table
        // sel a b : out
           0  0 ? : 0;
           0  1 ? : 1;
           1  ? 0 : 0;
           1  ? 1 : 1;
           x  0 0 : 0;
           x  1 1 : 1;
    endtable
endprimitive

// Sequential UDP
primitive d_latch (q, clk, d);
    output q;
    input clk, d;
    reg q;
    
    table
        // clk d : q : q+
           1   0 : ? : 0;
           1   1 : ? : 1;
           0   ? : ? : -; // no change
    endtable
endprimitive