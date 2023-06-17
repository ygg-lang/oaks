-- VHDL Test File - Comprehensive Syntax Coverage
-- This file tests various VHDL syntax elements for lexer testing

-- Library declarations
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;
use IEEE.STD_LOGIC_ARITH.ALL;
use IEEE.STD_LOGIC_UNSIGNED.ALL;
use IEEE.MATH_REAL.ALL;

library WORK;
use WORK.ALL;

library STD;
use STD.TEXTIO.ALL;

-- Custom package
package test_package is
    -- Constants
    constant DATA_WIDTH : integer := 32;
    constant ADDR_WIDTH : integer := 16;
    constant CLOCK_FREQ : real := 100.0e6; -- 100 MHz
    constant MAX_COUNT : integer := 1024;
    constant PI : real := 3.14159265359;
    
    -- Type declarations
    type state_type is (IDLE, READ, WRITE, DONE);
    type memory_array is array (0 to MAX_COUNT-1) of std_logic_vector(7 downto 0);
    type register_file is array (0 to 31) of std_logic_vector(DATA_WIDTH-1 downto 0);
    
    -- Enumerated types
    type color_type is (RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA);
    type direction_type is (NORTH, SOUTH, EAST, WEST);
    
    -- Record types
    type point_type is record
        x : integer;
        y : integer;
    end record;
    
    type complex_type is record
        real_part : real;
        imag_part : real;
    end record;
    
    type instruction_type is record
        opcode : std_logic_vector(7 downto 0);
        operand1 : std_logic_vector(15 downto 0);
        operand2 : std_logic_vector(15 downto 0);
        valid : std_logic;
    end record;
    
    -- Array types
    type byte_array is array (natural range <>) of std_logic_vector(7 downto 0);
    type integer_array is array (natural range <>) of integer;
    type real_array is array (natural range <>) of real;
    
    -- Subtype declarations
    subtype byte is std_logic_vector(7 downto 0);
    subtype word is std_logic_vector(15 downto 0);
    subtype dword is std_logic_vector(31 downto 0);
    subtype address_type is std_logic_vector(ADDR_WIDTH-1 downto 0);
    subtype data_type is std_logic_vector(DATA_WIDTH-1 downto 0);
    subtype small_integer is integer range 0 to 255;
    subtype normalized_real is real range 0.0 to 1.0;
    
    -- Function declarations
    function to_integer(signal_value : std_logic_vector) return integer;
    function to_std_logic_vector(int_value : integer; width : integer) return std_logic_vector;
    function reverse_bits(input_vector : std_logic_vector) return std_logic_vector;
    function count_ones(input_vector : std_logic_vector) return integer;
    function parity(input_vector : std_logic_vector) return std_logic;
    
    -- Procedure declarations
    procedure write_memory(
        signal clk : in std_logic;
        signal addr : in address_type;
        signal data : in data_type;
        signal mem : inout memory_array
    );
    
    procedure read_memory(
        signal clk : in std_logic;
        signal addr : in address_type;
        signal data : out data_type;
        signal mem : in memory_array
    );
    
    -- Component declarations
    component memory_controller is
        generic (
            DATA_WIDTH : integer := 32;
            ADDR_WIDTH : integer := 16
        );
        port (
            clk : in std_logic;
            reset : in std_logic;
            address : in std_logic_vector(ADDR_WIDTH-1 downto 0);
            write_data : in std_logic_vector(DATA_WIDTH-1 downto 0);
            read_data : out std_logic_vector(DATA_WIDTH-1 downto 0);
            write_enable : in std_logic;
            read_enable : in std_logic;
            ready : out std_logic;
            error : out std_logic
        );
    end component;
    
    component fifo is
        generic (
            DATA_WIDTH : integer := 8;
            DEPTH : integer := 16
        );
        port (
            clk : in std_logic;
            reset : in std_logic;
            push : in std_logic;
            pop : in std_logic;
            data_in : in std_logic_vector(DATA_WIDTH-1 downto 0);
            data_out : out std_logic_vector(DATA_WIDTH-1 downto 0);
            full : out std_logic;
            empty : out std_logic;
            count : out integer range 0 to DEPTH
        );
    end component;
    
end package test_package;

-- Package body
package body test_package is
    
    -- Function implementations
    function to_integer(signal_value : std_logic_vector) return integer is
        variable result : integer := 0;
    begin
        for i in signal_value'range loop
            result := result * 2;
            if signal_value(i) = '1' then
                result := result + 1;
            end if;
        end loop;
        return result;
    end function;
    
    function to_std_logic_vector(int_value : integer; width : integer) return std_logic_vector is
        variable result : std_logic_vector(width-1 downto 0);
        variable temp : integer := int_value;
    begin
        for i in 0 to width-1 loop
            if (temp mod 2) = 1 then
                result(i) := '1';
            else
                result(i) := '0';
            end if;
            temp := temp / 2;
        end loop;
        return result;
    end function;
    
    function reverse_bits(input_vector : std_logic_vector) return std_logic_vector is
        variable result : std_logic_vector(input_vector'range);
    begin
        for i in input_vector'range loop
            result(input_vector'high - i) := input_vector(i);
        end loop;
        return result;
    end function;
    
    function count_ones(input_vector : std_logic_vector) return integer is
        variable count : integer := 0;
    begin
        for i in input_vector'range loop
            if input_vector(i) = '1' then
                count := count + 1;
            end if;
        end loop;
        return count;
    end function;
    
    function parity(input_vector : std_logic_vector) return std_logic is
        variable result : std_logic := '0';
    begin
        for i in input_vector'range loop
            result := result xor input_vector(i);
        end loop;
        return result;
    end function;
    
    -- Procedure implementations
    procedure write_memory(
        signal clk : in std_logic;
        signal addr : in address_type;
        signal data : in data_type;
        signal mem : inout memory_array
    ) is
    begin
        if rising_edge(clk) then
            mem(to_integer(unsigned(addr(9 downto 0)))) <= data(7 downto 0);
        end if;
    end procedure;
    
    procedure read_memory(
        signal clk : in std_logic;
        signal addr : in address_type;
        signal data : out data_type;
        signal mem : in memory_array
    ) is
    begin
        if rising_edge(clk) then
            data <= (DATA_WIDTH-1 downto 8 => '0') & mem(to_integer(unsigned(addr(9 downto 0))));
        end if;
    end procedure;
    
end package body test_package;

-- Entity declaration
entity basic_test is
    generic (
        DATA_WIDTH : integer := 32;
        ADDR_WIDTH : integer := 16;
        CLOCK_PERIOD : time := 10 ns;
        RESET_ACTIVE : std_logic := '1';
        SIMULATION_TIME : time := 1000 ns
    );
    port (
        -- Clock and reset
        clk : in std_logic;
        reset : in std_logic;
        
        -- Data interface
        data_in : in std_logic_vector(DATA_WIDTH-1 downto 0);
        data_out : out std_logic_vector(DATA_WIDTH-1 downto 0);
        data_valid : in std_logic;
        data_ready : out std_logic;
        
        -- Address interface
        address : in std_logic_vector(ADDR_WIDTH-1 downto 0);
        
        -- Control signals
        enable : in std_logic;
        read_enable : in std_logic;
        write_enable : in std_logic;
        chip_select : in std_logic;
        output_enable : in std_logic;
        
        -- Status signals
        ready : out std_logic;
        busy : out std_logic;
        error : out std_logic;
        interrupt : out std_logic;
        
        -- Bidirectional signals
        data_bus : inout std_logic_vector(7 downto 0);
        
        -- Array ports
        control_signals : out std_logic_vector(7 downto 0);
        status_flags : in std_logic_vector(3 downto 0)
    );
end entity basic_test;

-- Architecture declaration
architecture behavioral of basic_test is
    
    -- Use the custom package
    use work.test_package.all;
    
    -- Signal declarations
    signal internal_data : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal internal_address : std_logic_vector(ADDR_WIDTH-1 downto 0);
    signal state : state_type;
    signal next_state : state_type;
    signal counter : integer range 0 to MAX_COUNT-1;
    signal timer : integer range 0 to 1000;
    signal memory : memory_array;
    signal registers : register_file;
    
    -- Clock and reset signals
    signal clk_internal : std_logic;
    signal reset_internal : std_logic;
    signal reset_sync : std_logic;
    
    -- Control signals
    signal enable_internal : std_logic;
    signal valid_internal : std_logic;
    signal ready_internal : std_logic;
    signal busy_internal : std_logic;
    signal error_internal : std_logic;
    
    -- Data processing signals
    signal data_buffer : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal address_buffer : std_logic_vector(ADDR_WIDTH-1 downto 0);
    signal checksum : std_logic_vector(7 downto 0);
    signal parity_bit : std_logic;
    
    -- Array signals
    signal byte_buffer : byte_array(0 to 15);
    signal word_buffer : std_logic_vector(15 downto 0);
    signal dword_buffer : std_logic_vector(31 downto 0);
    
    -- Record signals
    signal current_point : point_type;
    signal complex_number : complex_type;
    signal instruction : instruction_type;
    
    -- Enumerated signals
    signal current_color : color_type;
    signal direction : direction_type;
    
    -- Real and time signals
    signal frequency : real;
    signal period : time;
    signal phase : real;
    signal amplitude : real;
    
    -- File and text I/O
    file input_file : text open read_mode is "input.txt";
    file output_file : text open write_mode is "output.txt";
    
    -- Constants with different number formats
    constant BINARY_CONST : std_logic_vector(7 downto 0) := "10101010";
    constant HEX_CONST : std_logic_vector(15 downto 0) := X"DEAD";
    constant OCTAL_CONST : std_logic_vector(11 downto 0) := O"7654";
    constant DECIMAL_CONST : integer := 12345;
    constant REAL_CONST : real := 3.14159;
    constant TIME_CONST : time := 100 ns;
    constant STRING_CONST : string := "Hello VHDL";
    constant CHAR_CONST : character := 'A';
    constant BOOLEAN_CONST : boolean := true;
    
    -- Attributes
    signal data_length : integer := data_in'length;
    signal data_high : integer := data_in'high;
    signal data_low : integer := data_in'low;
    signal data_range : integer := data_in'range;
    signal data_left : integer := data_in'left;
    signal data_right : integer := data_in'right;
    
    -- Component instantiation signals
    signal mem_ctrl_ready : std_logic;
    signal mem_ctrl_error : std_logic;
    signal mem_ctrl_read_data : std_logic_vector(DATA_WIDTH-1 downto 0);
    
    signal fifo_full : std_logic;
    signal fifo_empty : std_logic;
    signal fifo_count : integer range 0 to 16;
    signal fifo_data_out : std_logic_vector(7 downto 0);
    
begin
    
    -- Concurrent signal assignments
    data_out <= internal_data when enable = '1' else (others => 'Z');
    ready <= ready_internal;
    busy <= busy_internal;
    error <= error_internal;
    data_ready <= valid_internal;
    
    -- Conditional signal assignments
    internal_address <= address when chip_select = '1' else (others => '0');
    
    -- Selected signal assignments
    with state select
        control_signals <= "00000001" when IDLE,
                          "00000010" when READ,
                          "00000100" when WRITE,
                          "00001000" when DONE,
                          "00000000" when others;
    
    -- Guarded signal assignments
    data_bus <= internal_data(7 downto 0) when output_enable = '1' else (others => 'Z');
    
    -- Generate statements
    gen_byte_buffers: for i in 0 to 15 generate
        byte_buffer(i) <= data_in((i+1)*8-1 downto i*8) when i < DATA_WIDTH/8 else (others => '0');
    end generate;
    
    gen_conditional: if DATA_WIDTH = 32 generate
        dword_buffer <= data_in;
    elsif DATA_WIDTH = 16 generate
        word_buffer <= data_in;
    end generate;
    
    -- Clock and reset processing
    clk_internal <= clk;
    reset_internal <= reset;
    
    -- Synchronous reset generation
    sync_reset_proc: process(clk, reset)
    begin
        if reset = RESET_ACTIVE then
            reset_sync <= '1';
        elsif rising_edge(clk) then
            reset_sync <= '0';
        end if;
    end process;
    
    -- State machine process
    state_machine_proc: process(clk, reset)
    begin
        if reset = RESET_ACTIVE then
            state <= IDLE;
            counter <= 0;
            timer <= 0;
            busy_internal <= '0';
            error_internal <= '0';
        elsif rising_edge(clk) then
            state <= next_state;
            
            -- Counter logic
            if enable = '1' then
                if counter < MAX_COUNT-1 then
                    counter <= counter + 1;
                else
                    counter <= 0;
                end if;
            end if;
            
            -- Timer logic
            if timer < 1000 then
                timer <= timer + 1;
            else
                timer <= 0;
            end if;
            
            -- Status signals
            case state is
                when IDLE =>
                    busy_internal <= '0';
                    error_internal <= '0';
                when READ | WRITE =>
                    busy_internal <= '1';
                    error_internal <= '0';
                when DONE =>
                    busy_internal <= '0';
                    error_internal <= '0';
                when others =>
                    busy_internal <= '0';
                    error_internal <= '1';
            end case;
        end if;
    end process;
    
    -- Next state logic
    next_state_proc: process(state, enable, data_valid, read_enable, write_enable)
    begin
        case state is
            when IDLE =>
                if enable = '1' then
                    if read_enable = '1' then
                        next_state <= READ;
                    elsif write_enable = '1' then
                        next_state <= WRITE;
                    else
                        next_state <= IDLE;
                    end if;
                else
                    next_state <= IDLE;
                end if;
                
            when READ =>
                if data_valid = '1' then
                    next_state <= DONE;
                else
                    next_state <= READ;
                end if;
                
            when WRITE =>
                next_state <= DONE;
                
            when DONE =>
                next_state <= IDLE;
                
            when others =>
                next_state <= IDLE;
        end case;
    end process;
    
    -- Data processing
    data_proc: process(clk)
        variable temp_data : std_logic_vector(DATA_WIDTH-1 downto 0);
        variable temp_address : std_logic_vector(ADDR_WIDTH-1 downto 0);
        variable temp_checksum : std_logic_vector(7 downto 0);
    begin
        if rising_edge(clk) then
            if reset_sync = '1' then
                internal_data <= (others => '0');
                data_buffer <= (others => '0');
                address_buffer <= (others => '0');
                checksum <= (others => '0');
                parity_bit <= '0';
                valid_internal <= '0';
                ready_internal <= '0';
            else
                -- Data processing logic
                if enable = '1' and data_valid = '1' then
                    temp_data := data_in;
                    temp_address := address;
                    
                    -- Calculate checksum
                    temp_checksum := (others => '0');
                    for i in 0 to DATA_WIDTH/8-1 loop
                        temp_checksum := temp_checksum + temp_data((i+1)*8-1 downto i*8);
                    end loop;
                    
                    -- Store processed data
                    internal_data <= temp_data;
                    data_buffer <= temp_data;
                    address_buffer <= temp_address;
                    checksum <= temp_checksum;
                    parity_bit <= parity(temp_data);
                    
                    valid_internal <= '1';
                else
                    valid_internal <= '0';
                end if;
                
                -- Ready signal logic
                if state = IDLE then
                    ready_internal <= '1';
                else
                    ready_internal <= '0';
                end if;
            end if;
        end if;
    end process;
    
    -- Memory operations
    memory_proc: process(clk)
    begin
        if rising_edge(clk) then
            if reset_sync = '1' then
                -- Initialize memory
                for i in 0 to MAX_COUNT-1 loop
                    memory(i) <= (others => '0');
                end loop;
                
                -- Initialize registers
                for i in 0 to 31 loop
                    registers(i) <= (others => '0');
                end loop;
            else
                -- Memory write
                if write_enable = '1' and chip_select = '1' then
                    if to_integer(unsigned(address)) < MAX_COUNT then
                        memory(to_integer(unsigned(address))) <= data_in(7 downto 0);
                    end if;
                end if;
                
                -- Register file operations
                if enable = '1' then
                    if to_integer(unsigned(address(4 downto 0))) < 32 then
                        registers(to_integer(unsigned(address(4 downto 0)))) <= data_in;
                    end if;
                end if;
            end if;
        end if;
    end process;
    
    -- Arithmetic and logic operations
    alu_proc: process(data_in, address, enable)
        variable a, b, result : std_logic_vector(DATA_WIDTH-1 downto 0);
        variable operation : std_logic_vector(3 downto 0);
    begin
        a := data_in;
        b := (others => '0');
        b(ADDR_WIDTH-1 downto 0) := address;
        operation := address(3 downto 0);
        
        case operation is
            when "0000" => -- ADD
                result := std_logic_vector(unsigned(a) + unsigned(b));
            when "0001" => -- SUB
                result := std_logic_vector(unsigned(a) - unsigned(b));
            when "0010" => -- AND
                result := a and b;
            when "0011" => -- OR
                result := a or b;
            when "0100" => -- XOR
                result := a xor b;
            when "0101" => -- NOT
                result := not a;
            when "0110" => -- NAND
                result := not (a and b);
            when "0111" => -- NOR
                result := not (a or b);
            when "1000" => -- SHL
                result := std_logic_vector(shift_left(unsigned(a), to_integer(unsigned(b(4 downto 0)))));
            when "1001" => -- SHR
                result := std_logic_vector(shift_right(unsigned(a), to_integer(unsigned(b(4 downto 0)))));
            when "1010" => -- ROL
                result := std_logic_vector(rotate_left(unsigned(a), to_integer(unsigned(b(4 downto 0)))));
            when "1011" => -- ROR
                result := std_logic_vector(rotate_right(unsigned(a), to_integer(unsigned(b(4 downto 0)))));
            when others =>
                result := a;
        end case;
        
        if enable = '1' then
            -- Output result (this would be connected to appropriate signals)
        end if;
    end process;
    
    -- Record and enumerated type operations
    record_proc: process(clk)
    begin
        if rising_edge(clk) then
            if reset_sync = '1' then
                current_point.x <= 0;
                current_point.y <= 0;
                complex_number.real_part <= 0.0;
                complex_number.imag_part <= 0.0;
                instruction.opcode <= (others => '0');
                instruction.operand1 <= (others => '0');
                instruction.operand2 <= (others => '0');
                instruction.valid <= '0';
                current_color <= RED;
                direction <= NORTH;
            else
                if enable = '1' then
                    -- Update point
                    current_point.x <= to_integer(signed(data_in(15 downto 0)));
                    current_point.y <= to_integer(signed(data_in(31 downto 16)));
                    
                    -- Update complex number
                    complex_number.real_part <= real(to_integer(signed(data_in(15 downto 0))));
                    complex_number.imag_part <= real(to_integer(signed(data_in(31 downto 16))));
                    
                    -- Update instruction
                    instruction.opcode <= data_in(7 downto 0);
                    instruction.operand1 <= data_in(23 downto 8);
                    instruction.operand2 <= data_in(31 downto 16);
                    instruction.valid <= data_valid;
                    
                    -- Cycle through colors
                    case current_color is
                        when RED => current_color <= GREEN;
                        when GREEN => current_color <= BLUE;
                        when BLUE => current_color <= YELLOW;
                        when YELLOW => current_color <= CYAN;
                        when CYAN => current_color <= MAGENTA;
                        when MAGENTA => current_color <= RED;
                        when others => current_color <= RED;
                    end case;
                    
                    -- Cycle through directions
                    case direction is
                        when NORTH => direction <= EAST;
                        when EAST => direction <= SOUTH;
                        when SOUTH => direction <= WEST;
                        when WEST => direction <= NORTH;
                        when others => direction <= NORTH;
                    end case;
                end if;
            end if;
        end if;
    end process;
    
    -- File I/O process
    file_io_proc: process
        variable input_line : line;
        variable output_line : line;
        variable data_value : integer;
        variable char_value : character;
        variable string_value : string(1 to 20);
    begin
        -- Read from file
        if not endfile(input_file) then
            readline(input_file, input_line);
            read(input_line, data_value);
            -- Process data_value
        end if;
        
        -- Write to file
        write(output_line, string'("Data: "));
        write(output_line, to_integer(unsigned(data_in)));
        write(output_line, string'(" at time "));
        write(output_line, now);
        writeline(output_file, output_line);
        
        wait for 100 ns;
    end process;
    
    -- Assert statements for verification
    assert_proc: process(clk)
    begin
        if rising_edge(clk) then
            -- Check data width
            assert DATA_WIDTH > 0
                report "DATA_WIDTH must be positive"
                severity error;
            
            -- Check address width
            assert ADDR_WIDTH > 0
                report "ADDR_WIDTH must be positive"
                severity error;
            
            -- Check counter bounds
            assert counter < MAX_COUNT
                report "Counter overflow detected"
                severity warning;
            
            -- Check state validity
            assert state /= IDLE or ready_internal = '1'
                report "Ready should be high in IDLE state"
                severity error;
        end if;
    end process;
    
    -- Component instantiations
    memory_controller_inst: memory_controller
        generic map (
            DATA_WIDTH => DATA_WIDTH,
            ADDR_WIDTH => ADDR_WIDTH
        )
        port map (
            clk => clk,
            reset => reset,
            address => address,
            write_data => data_in,
            read_data => mem_ctrl_read_data,
            write_enable => write_enable,
            read_enable => read_enable,
            ready => mem_ctrl_ready,
            error => mem_ctrl_error
        );
    
    fifo_inst: fifo
        generic map (
            DATA_WIDTH => 8,
            DEPTH => 16
        )
        port map (
            clk => clk,
            reset => reset,
            push => write_enable,
            pop => read_enable,
            data_in => data_in(7 downto 0),
            data_out => fifo_data_out,
            full => fifo_full,
            empty => fifo_empty,
            count => fifo_count
        );
    
end architecture behavioral;

-- Alternative architecture
architecture structural of basic_test is
    
    -- Component declarations
    component alu is
        generic (
            WIDTH : integer := 32
        );
        port (
            a : in std_logic_vector(WIDTH-1 downto 0);
            b : in std_logic_vector(WIDTH-1 downto 0);
            operation : in std_logic_vector(3 downto 0);
            result : out std_logic_vector(WIDTH-1 downto 0);
            zero : out std_logic;
            carry : out std_logic;
            overflow : out std_logic
        );
    end component;
    
    component register_file is
        generic (
            DATA_WIDTH : integer := 32;
            NUM_REGS : integer := 32
        );
        port (
            clk : in std_logic;
            reset : in std_logic;
            read_addr1 : in std_logic_vector(4 downto 0);
            read_addr2 : in std_logic_vector(4 downto 0);
            write_addr : in std_logic_vector(4 downto 0);
            write_data : in std_logic_vector(DATA_WIDTH-1 downto 0);
            write_enable : in std_logic;
            read_data1 : out std_logic_vector(DATA_WIDTH-1 downto 0);
            read_data2 : out std_logic_vector(DATA_WIDTH-1 downto 0)
        );
    end component;
    
    -- Internal signals
    signal alu_result : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal alu_zero : std_logic;
    signal alu_carry : std_logic;
    signal alu_overflow : std_logic;
    signal reg_read_data1 : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal reg_read_data2 : std_logic_vector(DATA_WIDTH-1 downto 0);
    
begin
    
    -- Structural connections
    data_out <= alu_result when enable = '1' else (others => 'Z');
    ready <= not busy;
    
    -- ALU instantiation
    alu_inst: alu
        generic map (
            WIDTH => DATA_WIDTH
        )
        port map (
            a => data_in,
            b => reg_read_data1,
            operation => address(3 downto 0),
            result => alu_result,
            zero => alu_zero,
            carry => alu_carry,
            overflow => alu_overflow
        );
    
    -- Register file instantiation
    reg_file_inst: register_file
        generic map (
            DATA_WIDTH => DATA_WIDTH,
            NUM_REGS => 32
        )
        port map (
            clk => clk,
            reset => reset,
            read_addr1 => address(9 downto 5),
            read_addr2 => address(14 downto 10),
            write_addr => address(4 downto 0),
            write_data => data_in,
            write_enable => write_enable,
            read_data1 => reg_read_data1,
            read_data2 => reg_read_data2
        );
    
    -- Status signal assignments
    error <= alu_overflow;
    busy <= not ready;
    
end architecture structural;

-- Configuration declaration
configuration basic_test_config of basic_test is
    for behavioral
        for memory_controller_inst : memory_controller
            use entity work.memory_controller(rtl);
        end for;
        for fifo_inst : fifo
            use entity work.fifo(behavioral);
        end for;
    end for;
end configuration basic_test_config;

-- Testbench entity
entity testbench is
end entity testbench;

-- Testbench architecture
architecture test of testbench is
    
    -- Constants
    constant CLOCK_PERIOD : time := 10 ns;
    constant DATA_WIDTH : integer := 32;
    constant ADDR_WIDTH : integer := 16;
    
    -- Signals
    signal clk : std_logic := '0';
    signal reset : std_logic := '1';
    signal data_in : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal data_out : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal address : std_logic_vector(ADDR_WIDTH-1 downto 0);
    signal enable : std_logic;
    signal data_valid : std_logic;
    signal data_ready : std_logic;
    signal read_enable : std_logic;
    signal write_enable : std_logic;
    signal chip_select : std_logic;
    signal output_enable : std_logic;
    signal ready : std_logic;
    signal busy : std_logic;
    signal error : std_logic;
    signal interrupt : std_logic;
    signal data_bus : std_logic_vector(7 downto 0);
    signal control_signals : std_logic_vector(7 downto 0);
    signal status_flags : std_logic_vector(3 downto 0);
    
    -- Test variables
    signal test_complete : boolean := false;
    
begin
    
    -- Clock generation
    clk_gen: process
    begin
        while not test_complete loop
            clk <= '0';
            wait for CLOCK_PERIOD / 2;
            clk <= '1';
            wait for CLOCK_PERIOD / 2;
        end loop;
        wait;
    end process;
    
    -- Reset generation
    reset_gen: process
    begin
        reset <= '1';
        wait for 100 ns;
        reset <= '0';
        wait;
    end process;
    
    -- Test stimulus
    stimulus: process
    begin
        -- Initialize signals
        data_in <= (others => '0');
        address <= (others => '0');
        enable <= '0';
        data_valid <= '0';
        read_enable <= '0';
        write_enable <= '0';
        chip_select <= '0';
        output_enable <= '0';
        status_flags <= "0000";
        
        -- Wait for reset
        wait until reset = '0';
        wait for 10 * CLOCK_PERIOD;
        
        -- Test 1: Basic write operation
        report "Starting Test 1: Basic write operation";
        wait until rising_edge(clk);
        enable <= '1';
        chip_select <= '1';
        write_enable <= '1';
        data_in <= X"12345678";
        address <= X"0100";
        data_valid <= '1';
        
        wait until rising_edge(clk);
        write_enable <= '0';
        data_valid <= '0';
        
        wait for 5 * CLOCK_PERIOD;
        
        -- Test 2: Basic read operation
        report "Starting Test 2: Basic read operation";
        wait until rising_edge(clk);
        read_enable <= '1';
        address <= X"0100";
        
        wait until rising_edge(clk);
        read_enable <= '0';
        
        wait for 5 * CLOCK_PERIOD;
        
        -- Test 3: Multiple data transfers
        report "Starting Test 3: Multiple data transfers";
        for i in 0 to 15 loop
            wait until rising_edge(clk);
            write_enable <= '1';
            data_in <= std_logic_vector(to_unsigned(i * 16#1000#, DATA_WIDTH));
            address <= std_logic_vector(to_unsigned(i, ADDR_WIDTH));
            data_valid <= '1';
            
            wait until rising_edge(clk);
            write_enable <= '0';
            data_valid <= '0';
            
            wait for 2 * CLOCK_PERIOD;
        end loop;
        
        -- Test 4: Error conditions
        report "Starting Test 4: Error conditions";
        wait until rising_edge(clk);
        enable <= '1';
        write_enable <= '1';
        read_enable <= '1'; -- Simultaneous read and write
        data_in <= X"DEADBEEF";
        address <= X"FFFF";
        data_valid <= '1';
        
        wait until rising_edge(clk);
        write_enable <= '0';
        read_enable <= '0';
        data_valid <= '0';
        
        wait for 10 * CLOCK_PERIOD;
        
        -- Test 5: Timing tests
        report "Starting Test 5: Timing tests";
        for i in 0 to 7 loop
            wait until rising_edge(clk);
            enable <= '1';
            data_in <= std_logic_vector(shift_left(to_unsigned(1, DATA_WIDTH), i));
            address <= std_logic_vector(to_unsigned(i, ADDR_WIDTH));
            data_valid <= '1';
            
            wait until data_ready = '1';
            enable <= '0';
            data_valid <= '0';
            
            wait for CLOCK_PERIOD;
        end loop;
        
        -- End of test
        wait for 50 * CLOCK_PERIOD;
        report "All tests completed";
        test_complete <= true;
        wait;
    end process;
    
    -- Monitor process
    monitor: process
    begin
        wait until reset = '0';
        
        while not test_complete loop
            wait until rising_edge(clk);
            
            if data_ready = '1' then
                report "Data ready: " & 
                       "data_out=" & to_hstring(data_out) & 
                       " address=" & to_hstring(address) &
                       " at time " & time'image(now);
            end if;
            
            if error = '1' then
                report "Error detected at time " & time'image(now)
                    severity warning;
            end if;
        end loop;
        
        wait;
    end process;
    
    -- Device under test instantiation
    dut: entity work.basic_test(behavioral)
        generic map (
            DATA_WIDTH => DATA_WIDTH,
            ADDR_WIDTH => ADDR_WIDTH,
            CLOCK_PERIOD => CLOCK_PERIOD,
            RESET_ACTIVE => '1',
            SIMULATION_TIME => 10000 ns
        )
        port map (
            clk => clk,
            reset => reset,
            data_in => data_in,
            data_out => data_out,
            data_valid => data_valid,
            data_ready => data_ready,
            address => address,
            enable => enable,
            read_enable => read_enable,
            write_enable => write_enable,
            chip_select => chip_select,
            output_enable => output_enable,
            ready => ready,
            busy => busy,
            error => error,
            interrupt => interrupt,
            data_bus => data_bus,
            control_signals => control_signals,
            status_flags => status_flags
        );
    
end architecture test;