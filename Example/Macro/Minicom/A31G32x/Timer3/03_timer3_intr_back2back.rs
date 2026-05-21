# A. Description
#    A list of commands here configures timer3 and generates a back-to-back interrupt (Period, A, B, C, Bottom).
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer3
#
# D. Default Port
#    1. Timer30 : PA8 (PWM AA Output)
#               : PA7 (PWM AB Output)
#               : PA9 (PWM BA Output)
#               : PB0 (PWM BB Output)
#               : PA10 (PWM CA Output)
#               : PB1 (PWM CB Output)
#               : PA12/PA12 (Capture Input/External Clock Input)
#               : PA6 (BLANK Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer30
# 1. Channel                   : 0 (Timer30)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 48MHz )
#    Source Divide             : 10 
#    Timer3 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i b s 19200 4800 9600 14400 ]
#    Operation                 : i (Interrupt)
#    Mode                      : b (BackToBack)
#    Output Mode               : s (6-ch)
#    Period                    : 19200
#    A Data                    : 4800
#    B Data                    : 9600
#    C Data                    : 14400
#
# Timer30
send ""

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

send "clk 0 m h 10 1000"
expect {
    "<TIMER3> # "
    break
    timeout 5 goto end
}

send "config 0 i b s 19200 4800 9600 14400"
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
