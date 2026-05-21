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
#    1. Timer30 : PE0 (PWM AA Output)
#               : PE1 (PWM AB Output)
#               : PE2 (PWM BA Output)
#               : PE3 (PWM BB Output)
#               : PE4 (PWM CA Output)
#               : PE5 (PWM CB Output)
#               : PF7 (Capture Input)
#               : PF2 (External Clock Input)
#               : PF5 (BLANK Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer30
# 1. Channel                   : 0 (Timer30)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide             : 10 
#    Timer3 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i b s 12800 3200 6400 9600 ]
#    Operation                 : i (Interrupt)
#    Mode                      : b (BackToBack)
#    Output Mode               : s (6-ch)
#    Period                    : 12800
#    A Data                    : 3200
#    B Data                    : 6400
#    C Data                    : 9600
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

send "config 0 i b s 12800 3200 6400 9600"
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
