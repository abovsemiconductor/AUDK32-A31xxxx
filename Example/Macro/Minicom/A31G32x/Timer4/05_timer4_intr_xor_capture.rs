# A. Description
#    A list of commands here configures timer4 and generates a capture interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer4
#    2. Timer1
#    3. PCU/GPIO
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
#    3. Timer11 : PA6 (Output/Capture Input/External Clock Input)
#
#    4. Timer12 : PA7 (Output/Capture Input/External Clock Input)
#
# D. Port Connection
#    1. PB14 (Timer10 Output) ----> PB4 (Timer40 Capture Input 1)
#    2. PA6 (Timer11 Output) ----> PC7 (Timer40 Capture Input 2)
#    3. PA7 (Timer12 Output) ----> PC8 (Timer40 Capture Input 3)
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
#    Timer2 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i c r x 0 h 19200 19200 ]
#    Operation                 : i (Interrupt)
#    Mode                      : c (Capture)
#    Capture Egde              : r (Rising)
#    Capture Source            : x (XOR Capture Port Inputs)
#    Counter Value             : 0 (Prescaler + 1)
#    Output Port Polarity      : h (High)
#    A Data                    : 19200
#    B Data                    : 192200
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 48MHz )
#    Source Divide             : 10 
#    Timer1 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i p h 48000 48000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Periodic)
#    Output Port Polarity      : h (High)
#    A Data                    : 48000
#    B Data                    : 48000
#
# Timer11
# 1. Channel                   : 1 (Timer11)
#
# 2. Clock                     : [ 1 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 48MHz )
#    Source Divide             : 10 
#    Timer2 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 1 i p h 48000 48000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 48000
#    B Data                    : 48000
#
# Timer12
# 1. Channel                   : 2 (Timer12)
#
# 2. Clock                     : [ 2 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 48MHz )
#    Source Divide             : 10 
#    Timer2 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 2 i p h 4800 4800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 4800
#    B Data                    : 4800
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 6 a 6 ] [ 0 7 a 6 ]
#    Pin Number                : 6 / 7
#    Pin Mode                  : a (Alternative)
#    Alternative               : 6 / 6 (T11OUT / T12OUT)
# 
# PCU (PA6)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 6 a 6"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PA7)
send "port 0 7 a 6"
expect {
    "<PCU> # "
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

send "clk 0 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 48000 48000"
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


# Timer11
send "uninit 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 1 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 1 i p h 48000 48000"
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


# Timer12
send "uninit 2"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 2"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 2 m h 10 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 2 i p h 4800 4800"
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

send "clk 0 m h 10 1000"
expect {
    "<TIMER4> # "
    break
    timeout 5 goto end
}

send "config 0 i c r x 0 h 19200 19200"
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


# Timer1
send "cm timer1"
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
sleep 0.4
send "start 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}
sleep 0.4
send "start 2"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

end:
