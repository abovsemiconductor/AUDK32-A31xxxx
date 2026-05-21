# A. Description
#    A list of commands here configures timer2 and generates a periodic interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer2
#    2. Timer1
#
# D. Default Port
#    1. Timer20 : PC0/PC0 (Output/Capture Input)
#               : PC2 (External Clock Input)
#    2. Timer10 : PE6/PE6 (Output/Capture Input)
#               : PC11 (External Clock Input)
#
# E. Port Connection
#    1. PE6 (Timer10 Output) ----> PC2 (Timer20 External Clock Input)
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
# 3. Config                    : [ 0 i p h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Periodic)
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

send "clk 0 e f 0"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 4800 4800"
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


# Timer10
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

end:
