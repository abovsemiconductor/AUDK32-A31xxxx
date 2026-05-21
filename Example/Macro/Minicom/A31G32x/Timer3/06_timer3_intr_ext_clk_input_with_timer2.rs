# A. Description
#    A list of commands here configures timer3 and generates a periodic interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer3
#    2. Timer2
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
#    1. Timer20 : PB3 (Output)
#               : PB10 (Capture Input)
#               : PA5 (External Clock Input)
#
# E. Port Connection
#    1. PB3 (Timer20 Output) ----> PA12 (Timer30 External Clock Input)
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
# 3. Config                    : [ 0 i i s 19200 4800 9600 14400 ]
#    Operation                 : i (Interrupt)
#    Mode                      : i (Interval)
#    Output Mode               : s (6-ch)
#    Period                    : 19200
#    A Data                    : 4800
#    B Data                    : 9600
#    C Data                    : 14400
#
# Timer20 
# 1. Channel                   : 0 (Timer20)
#
# 2. Clock                     : [ 0 m h 10 100 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide             : 10 
#    Timer2 Pre-Divide         : 100 (MCCR Clock / 10) / 100
#
# 3. Config                    : [ 0 i p h 5 5 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 5
#    B Data                    : 5
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

send "clk 0 m h 10 100"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 5 5"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "log off"
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

send "config 0 i i s 19200 4800 9600 14400"
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
