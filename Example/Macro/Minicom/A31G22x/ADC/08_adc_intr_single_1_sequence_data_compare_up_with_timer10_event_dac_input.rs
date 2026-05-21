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
#    2. DAC0    : PA6 (DAO)
#
# E. Port Connection
#    1. PA6 (DAC0 DAO) ---> PA0 (AN0)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 64 ]
#    Source                     : p (Periperhal Clock)
#    Source Divide              : 64
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
# 5. Cmp                        : [ 0 e 0 -intr e u ]
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
# Timer11
# 1. Channel                    : 1 (Timer11)
#
# 2. Clock                      : [ 1 m h 10 1000 ]
#    Source                     : m (MCCR : Misc Clock)
#    MCCR Source                : h (HSI : High Speed Internal Clock - 32MHz)
#    Source Divide              : 10
#    Timer1 Pre-Divide          : 1000 (MCCR Clock / 10) / 1000
#
# 3. Config                     : [ 1 i p h 1600 1600 ]
#    Operation                  : i (Interrupt)
#    Mode                       : p (Period)
#    Output Port Polarity       : h (High)
#    A Data                     : 1600
#    B Data                     : 1600
#
# DAC0
# 1. Config                     : [ 0 d m i 1 -port 0x01 ]
#    Operation                  : d (Interrupt DMA)
#    Mode                       : m (Manual)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : 1 (Timer11)
#    Output Port                : 1 (DAC0)
#
# PCU (PA6)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 6 a 3 ]
#    Pin Number                : 6
#    Pin Mode                  : a (Alternative)
#    Alternative               : 3 (DAC Output)
# 
# PCU (PA6)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 6 a 3"
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

send "log off"
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

send "config 0 i p h 1600 1600"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer11
send "cm timer1"
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

send "config 1 i p h 1600 1600"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# ADC0
send "cm adc"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "clk 0 p 64"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i s i 1 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 0 0 1 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "cmp 0 e 0 1024 -intr e u"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<ADC> # "
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

send "start 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}


# Timer11
send "start 1"
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

sleep 0.5
send "config 0 d m i 1 -port 0x01"
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


# ADC0
send "cm adc"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}


# DAC0
sleep 15
send "cm dac"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "buf 0 0 10 0xF000 0xE000 0xD000 0xC000 0xB000 0xA000 0x9000 0x8000 0x7000 0x6000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "buf 0 10 6 0x5000 0x4000 0x3000 0x2000 0x1000 0x0000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "buf 0 16 10 0x1000 0x2000 0x3000 0x4000 0x5000 0x6000 0x7000 0x8000 0x9000 0xA000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "buf 0 26 6 0xB000 0xC000 0xD000 0xE000 0xF000 0xFFF0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "start 0"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

end:

