# A. Description
#    A list of commands here configures SCU CLK and generates a certain frequency of PLL Clock.
#
# B. Preparation
#    Make an environment to measure clock frequency by clock output port.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUCLK
#    2. PCU/GPIO
#
# D. Default Port
#    1. PCU    : PF0 (Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PF0)
# 1. Port                        : [ 5 0 a 1 ]
#    Port Group Number           : 5 (F group)
#    Pin Number                  : 0
#    Pin Mode                    : Alternative (Mux)
#    Pin Alt Number              : 1
#
# SCUCLK
# 1. Clkout                      : [ m 0 e ]
#    Clock Source                : m (MCLK : Main Clock)
#    Clock Source Divide         : 0
#    Clock Output Enable         : e (Enable)
#
# 2. PLL                         : [ e h 8 2 15 0 0 s 10 2 ]
#    Enable                      : e (Enable)
#    Clock Source                : h (HSI : High Speed Internal Clock) ( 48MHz )
#    Clock Divide                : 8
#    PLL Pre-divide (R)          : 2
#    PLL Divide 1   (N1)         : 15
#    PLL Divide 2   (N2)         : 0
#    PLL Output Divide (P)       : 0
#    PLL VCO (Freq Doubler) (D)  : s (VCO)
#    PLL Current                 : 10
#    PLL Bias                    : 2 (none)
#
#              ( FIN * ( N1 + 1 ) * ( D + 1 ))
#    Fout = -------------------------------------
#            (( R + 1 ) * ( N2 + 1 ) * ( P + 1 ))
#
# ex) pll e h 8 2 15 0 0 s 10 2 ( HSI 48MHz )
#
#            ((48000000 / 8) * ( 15 + 1 ) * ( 0 + 1 ))
#    Fout = -------------------------------------------,    Fout = 32MHz
#               (( 2 + 1 ) * ( 0 + 1 ) * ( 0 + 1 ))
#
# ex) pll e e 4 0 15 0 0 s 10 2 ( HSE : High Speed External Clock = 8MHz )
#
#            ((8000000 / 4) * ( 15 + 1 ) * ( 0 + 1 ))
#    Fout = -------------------------------------------,    Fout = 32MHz
#               (( 0 + 1 ) * ( 0 + 1 ) * ( 0 + 1 ))
#
#
# PCU (PF4)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 5 0 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# SCUCLK
send "cm scuclk"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

send "clkout m 0 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}


# PLL configuration 32MHz 
send "pll e h 8 2 15 0 0 s 10 2"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

end:
