# A. Description
#    A list of commands here configures timer3 and generates a interval interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer3
#    2. SCUCLK
#    3. PCU/GPIO
#
# D. Default Port
#    1. Timer30 : PE0 (PWM AA Output)
#               : PE1 (PWM AB Output)
#               : PE2 (PWM BA Output)
#               : PE3 (PWM BB Output)
#               : PE4 (PWM CA Output)
#               : PE5 (PWM CB Output)
#               : PF7 (Capture Input)
#               : PF2 (External Clock Input)
#               : PF5 (BLANK Input)
#    2. PCU     : PF4 (Clock Output)
#
# E. Port Connection
#    1. PF4 (Clock Output) ----> PF2 (Timer30 External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer30
# 1. Channel                   : 0 (Timer30)
#
# 2. Clock                     : [ 0 e f 0 ]
#    Source                    : e (External Clock Input) 
#    External Clock Input Edge : f (Falling) 
#    Timer3 Pre-Divide         : 0 (No Pre-Divider available so default 0)
#
# 3. Config                    : [ 0 i i s 64000 14000 34000 54000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : i (Interval)
#    Output Mode               : s (6-ch)
#    Period                    : 64000
#    A Data                    : 14000
#    B Data                    : 34000
#    C Data                    : 54000
#
# PCU (PF4)
# 1. Port Group                : 5 (PCU Port F)
#
# 2. Port                      : [ 5 4 a 1 ]
#    Pin Number                : 4
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Clock Output)
#
# SCUCLK
# 1. Clock Out                 : [ clkout m 15 e ]  
#    Clock Source              : m (MCLK - HSI 32MHz)
#    Clock Source Divide       : 15 (Clock Source / (2 * (Divide + 1))
#    Clock Output Enable       : e (Enable)
#
# PCU (PF4)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 5 4 a 1"
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

send "clkout m 15 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

# Timer30
send "cm timer3"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "clk 0 e f 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "config 0 i i s 64000 14000 34000 54000"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

end:
