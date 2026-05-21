# A. Description
#    A list of commands here configures timer2 and generates a periodic interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer2
#    2. SCUCLK
#    3. PCU/GPIO
#
# D. Default Port
#    1. Timer20 : PB3 (Output)
#               : PB10 (Capture Input)
#               : PA5 (External Clock Input)
#    2. PCU     : PF0 (Clock Output)
#
# E. Port Connection
#    1. PF0 (Clock Output) ----> PA5 (Timer20 External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer20
# 1. Channel                   : 0 (Timer20)
#
# 2. Clock                     : [ 0 e f 0 ]
#    Source                    : e (External Clock Input) 
#    External Clock Input Edge : f (Falling) 
#    Timer1 Pre-Divide         : 0 (No Pre-Divider available so default 0)
#
# 3. Config                    : [ 0 i p h 2000000 2000000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Periodic)
#    Output Port Polarity      : h (High)
#    A Data                    : 2000000
#    B Data                    : 2000000
#
# PCU (PF4)
# 1. Port Group                : 5 (PCU Port F)
#
# 2. Port                      : [ 5 0 a 1 ]
#    Pin Number                : 0
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Clock Output)
#
# SCUCLK
# 1. Clock Out                 : [ clkout m 11 e ]  
#    Clock Source              : m (MCLK - HSI 48MHz)
#    Clock Source Divide       : 11 (Clock Source / (2 * (Divide + 1))
#    Clock Output Enable       : e (Enable)
#
# PCU
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

send "clkout m 11 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}


# Timer20
send "cm timer2"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "clk 0 e f 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 2000000 2000000"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

end:
