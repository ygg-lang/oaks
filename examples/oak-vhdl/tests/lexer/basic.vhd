-- VHDL test file
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity counter is
    Port ( 
        clk : in STD_LOGIC;
        reset : in STD_LOGIC;
        enable : in STD_LOGIC;
        count : out STD_LOGIC_VECTOR (7 downto 0)
    );
end counter;

architecture Behavioral of counter is
    signal count_int : unsigned(7 downto 0) := (others => '0');
begin
    process(clk, reset)
    begin
        if reset = '1' then
            count_int <= (others => '0');
        elsif rising_edge(clk) then
            if enable = '1' then
                count_int <= count_int + 1;
            end if;
        end if;
    end process;
    
    count <= std_logic_vector(count_int);
end Behavioral;

-- Test bench
entity counter_tb is
end counter_tb;

architecture testbench of counter_tb is
    component counter
        Port ( 
            clk : in STD_LOGIC;
            reset : in STD_LOGIC;
            enable : in STD_LOGIC;
            count : out STD_LOGIC_VECTOR (7 downto 0)
        );
    end component;
    
    signal clk_tb : STD_LOGIC := '0';
    signal reset_tb : STD_LOGIC := '0';
    signal enable_tb : STD_LOGIC := '0';
    signal count_tb : STD_LOGIC_VECTOR (7 downto 0);
    
    constant clk_period : time := 10 ns;
    
begin
    uut: counter port map (
        clk => clk_tb,
        reset => reset_tb,
        enable => enable_tb,
        count => count_tb
    );
    
    clk_process: process
    begin
        clk_tb <= '0';
        wait for clk_period/2;
        clk_tb <= '1';
        wait for clk_period/2;
    end process;
    
    stim_proc: process
    begin
        reset_tb <= '1';
        wait for 20 ns;
        reset_tb <= '0';
        enable_tb <= '1';
        wait for 100 ns;
        enable_tb <= '0';
        wait for 50 ns;
        enable_tb <= '1';
        wait;
    end process;
end testbench;