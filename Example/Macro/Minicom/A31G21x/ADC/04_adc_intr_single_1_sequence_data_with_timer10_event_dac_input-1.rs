# A. Description
#    A list of commands here configures ADC and generates single capture interrupt by DAC output signal.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
# 
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer1
#    3. DAC
#    4. PCU/GPIO
#
# D. Default Port
#    1. ADC0    : PA0 (AN0)
#               : PA1 (AN1)
#               : PA2 (AN2)
#    2. DAC0    : PA7 (DAO)
#
# E. Port Connection
#    1. PA7 (DAC0 DAO) ---> PA0 (AN0)
#
# F.Note 
#    1. Modify the line '#define __ROM_SIZE 0x00008000' to '#define __ROM_SIZE 0x00010000 in the ac5_a31g21x.sct file.
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 63 ]
#    Source                     : p (Periperhal Clock)
#    Peripheral Clock Divide    : 63 (Divide + 1)
#
# 3. Config                     : [ config 0 i s i 1 1 0 ]
#    Operation                  : i (Interrupt)
#    Mode                       : s (Single)
#    Reference Level            : i (Internal VDD)
#    Sequence Count             : 1
#    Base Trigger Source        : 1 (Timer1)
#    Sampling Time              : 0
#
# 4. Seq                        : [ 0 0 0 1 0 ]
#    Sequence Number            : 0
#    Input Channel              : 0
#    Trigger Source             : 1 (Timer1)
#    Trigger Source Number      : 0 (Timer10)
#
# Timer10
# 1. Channel                    : 0 (Timer10)
#
# 2. Clock                      : [ 0 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 0 i p h 1600 1600 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 1600
#    B Data                     : 1600
#
# DAC0
# 1. Config                     : [ 0 p m ign i c -port 0x01 -pg p 0]
#    Operation                  : p (Polling)
#    Mode                       : m (Manual)
#    Internal Test Procedure    : ign (Ignore)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Output Port                : 0x01 (DAC0 DAO)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# PCU (PA7)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 7 a 3 ]
#    Pin Number                : 7
#    Pin Mode                  : a (Alternative)
#    Alternative               : 3 (DAO)
# 

# DAC0
send ""

send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "data 0 0x0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x5"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x6"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x7"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x9"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xa"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xb"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xc"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xd"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xe"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xf"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x10"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x11"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x12"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x13"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x14"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x15"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x16"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x17"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x18"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x19"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1a"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1b"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1d"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1e"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1f"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

end:
