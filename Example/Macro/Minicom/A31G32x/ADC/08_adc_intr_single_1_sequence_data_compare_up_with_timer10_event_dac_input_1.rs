# A. Description
#    A list of commands here configures ADC and generates single capture and comparator matched interrupt by DAC output signal.
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
#    2. DAC0    : PA4 (DAO)
#
# E. Port Connection
#    1. PA4 (DAC0 DAO) ---> PA0 (AN0)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 64 ]
#    Source                     : p (Peripheral Clock)
#    Peripheral Clock Divide    : 64
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
# 5. Cmp                        : [ 0 e 0 1024 -intr e u ]
#    Comparator Enable          : e (Enable)
#    Input Channel              : 0 
#    Comparator Data            : 1024
#    Interrupt Enable           : e (Enable)
#    Interrupt Trigger          : u (Input Channel Sampling Data > Comparator Data)
#
# Timer10
# 1. Channel                    : 0 (Timer10)
#
# 2. Clock                      : [ 0 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 48MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 0 i p h 2400 2400 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 2400
#    B Data                     : 2400
#
# DAC0
# 1. Config                     : [ 0 p m ign i c -port 0x01 -pg p 0]
#    Operation                  : p (Polling)
#    Mode                       : m (Manual)
#    Internal Test Procedure    : ign (Ignore)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : c (Constant)
#    Output Port                : 0x01 (DAC_OUT1)
#    Programmable Gain          : p (Positive)
#    Programmable Gain Level    : 0 (0dB)
#
# PCU (PA4)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 4 a 10 ]
#    Pin Number                : 4
#    Pin Mode                  : a (Alternative)
#    Alternative               : 10 (DAC_OUT1)
# 
send ""

send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "data 0 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "data 0 0x14"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x28"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x64"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x78"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x8c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xa0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xB4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xc8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xdc"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0xf0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x104"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x118"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x12c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x140"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x154"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x168"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x17c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x190"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1A4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1B8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1cc"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1e0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x1f4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x208"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x21c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x230"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x244"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x258"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x26c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x280"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x294"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2a8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2bc"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2d0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2e4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x2f8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x30c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x320"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x334"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x348"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x35c"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x370"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x384"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x398"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3ac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3c0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3d4"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3e8"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x3fc"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "data 0 0x410"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.5

send "stop 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
