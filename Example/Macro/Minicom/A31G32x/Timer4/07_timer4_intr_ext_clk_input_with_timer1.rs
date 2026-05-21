# A. Description
#    A list of commands here configures timer4 and generates a periodic interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer4
#    2. Timer1
#
# D. Default Port
#    1. Timer40 : PA0 (Output)
#               : PB4 (Capture Input 1)
#               : PC7 (Capture Input 2)
#               : PC8 (Capture Input 3)
#               : PC9 (External Clock Input)
#
#    2. Timer10 : PB14 (Output)
#               : PA3 (Capture Input)
#               : PB15 (External Clock Input)
#
# E. Port Connection
#    1. PB14 (Timer10 Output) ----> PC9 (Timer40 External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer40
# 1. Channel                   : 0 (Timer40)
#
# 2. Clock                     : [ 0 e f 0 ]
#    Source                    : e (External Clock Input) 
#    External Clock Input Edge : f (Falling) 
#    Timer4 Pre-Divide         : 0 (No Pre-Divider available so default 0)
#
# 3. Config                    : [ 0 i p h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 4800
#    B Data                    : 4800
#
# Timer10
# 1. Channel                   : 0 (Timer10)
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
# Timer10
send ""

send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 10 100"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 5 5"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer40
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

send "clk 0 e f 0"
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
