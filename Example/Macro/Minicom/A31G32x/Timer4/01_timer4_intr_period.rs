# A. Description
#    A list of commands here configures timer4 and generates a period interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer4
#
# D. Default Port
#    1. Timer40 : PA0 (Output)
#               : PB4 (Capture Input 1)
#               : PC7 (Capture Input 2)
#               : PC8 (Capture Input 3)
#               : PC9 (External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer40
# 1. Channel                   : 0 (Timer40)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 48MHz )
#    Source Divide             : 10 
#    Timer4 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i p h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 4800
#    B Data                    : 4800
#
# Timer40
send ""

send "cm timer4"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 10 1000"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 4800 4800"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

end:
