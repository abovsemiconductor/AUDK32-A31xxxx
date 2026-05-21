# A. Description
#    A list of commands here configures timer2 and generates a capture interrupt.
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
#    1. PE6 (Timer10 Output) ----> PC0 (Timer20 Capture Input)
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
# 3. Config                    : [ 0 i c b e c h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : c (Capture)
#    Capture Egde              : b (Both)
#    Capture Source            : e (Capture Port Input)
#    Counter Mode              : c (Clear Count)
#    Output Port Polarity      : h (High)
#    A Data                    : 2400
#    B Data                    : 2400
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide             : 10 
#    Timer1 Pre-Divide         : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                    : [ 0 i p h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Periodic)
#    Output Port Polarity      : h (High)
#    A Data                    : 4800
#    B Data                    : 4800
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

send "clk 0 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 4800 4800"
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

send "clk 0 m h 10 1000"
expect {
    "<TIMER2> # "
    break
    timeout 5 goto end
}

send "config 0 i c b e c h 2400 2400"
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
