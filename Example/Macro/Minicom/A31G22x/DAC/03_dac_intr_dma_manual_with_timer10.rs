# A. Description
#    A list of commands here configures DAC and generates Rx done dma interrupt by specific data.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. DAC
#    2. Timer1
#
# D. Default Port
#    1. DAC0    : PA6 (DAO)
#  
# For more information, read a user's manual of the target device carefully.
#
# DAC0
# 1. Config                     : [ 0 d m i 0 -port 0x01 ]
#    Operation                  : d (Interrupt DMA)
#    Mode                       : m (Manual)
#    Reference Level            : i (Internal VDD)
#    Re-load reference          : 0 (Timer10)
#    Output Port                : 1
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 m h 10 1000]
#    Source                    : m ( MCCR : Misc Clock )
#    MCCR Source               : h ( HSI : High Speed Internal Clock - 32MHz )
#    Source Divide             : 10 
#    Timer1 Pre-Divide         : 1000 ( MCCR Clock / 10 ) / 1000
#
# 3. Config                    : [ 0 i p h 800 800 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 800
#    B Data                    : 800
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

send "config 0 i p h 800 800"
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

send "config 0 d m i 0 -port 0x01"
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
sleep 10

send "buf 0 0 10 0xF000 0xE000 0xD000 0xC000 0xB000 0xA000 0x9000 0x8000 0x7000 0x6000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "buf 0 10 6 0x5000 0x4000 0x3000 0x2000 0x1000 0x0000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "buf 0 16 10 0x1000 0x2000 0x3000 0x4000 0x5000 0x6000 0x7000 0x8000 0x9000 0xA000"
expect {
    "<DAC> # "
    break
    timeout 5 goto end
}

send "buf 0 26 6 0xB000 0xC000 0xD000 0xE000 0xF000 0xFFF0"
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

end:

