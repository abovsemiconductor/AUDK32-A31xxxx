# A. Description
#    A list of commands here configures CMP and generates both edge detected interrupt.
#
# B. Preparation
#    Supply reference and source signal to specific port by a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. CMP
#    2. DAC
#    3. Timer10
#
# D. Default Port
#    1. Timer10 : PE6 (Timer10 Output)
#    2. DAC     : PA6 (DAC0_OUT)
#    3. CMP     : PA2 (CP0A)
#               : PA7 (External Ref)
#
# E. Port Connection
#    1. PE6 (Timer10 Output) ----> PA2 (CP0A)
#    2. PA6 (DAC0_OUT) ----> PA7 (External Ref)
#
# For more information, read a user's manual of the target device carefully.
#
# CMP0
# 1. Config                     : [ 0 i m 0 0 h b ]
#    Operation                  : i (Interrupt)
#    Interrupt Mode             : m (Maskable)
#    Source                     : 0 (CP0A)
#    Reference                  : 0 (External Ref)
#    Hysteresis                 : h (High)
#    Trigger Mode               : b (both edge)
#
# DAC0
# 1. Config                     : [ 0 p m ign i c -port 0x01 -pg p 0]
#    Operation                  : p (Polling)
#    Mode                       : m (Manual)
#    Internal Test Procedure    : ign (Ignore)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Output Port                : 0x01 (DAC0_OUT)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 m h 10 1000 ]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 32MHz )
#    Source Divide             : 10 
#    Timer1 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i p h 3200 3200 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 3200
#    B Data                    : 3200
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

send "config 0 i p h 3200 3200"
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


# DAC0
send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "config 0 p m ign i c -port 0x01 -pg p 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "data 0 0xAFF"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}


# CMP0
send "cm cmp"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "config 0 i m 0 0 h b"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "log on 1"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<CMP> # "
    break
    timeout 5 goto end
}

end:
