# A. Description
#    A list of commands here configures timer2 and generates a PWM interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer2
#
# D. Default Port
#    1. Timer20 : PC0/PC0 (Output/Capture Input)
#               : PC2 (External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer20
# 1. Channel                   : 0 (Timer20)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide             : 10 
#    Timer2 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i o h 4800 9600 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (PWM) 
#    Output Port Polarity      : h (High)
#    A Data                    : 4800
#    B Data                    : 9600
#
# Timer20
send ""

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

send "clk 0 m h 10 1000"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i m h 4800 9600"
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
