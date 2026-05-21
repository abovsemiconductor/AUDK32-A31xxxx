# A. Description
#    A list of commands here configures LED module as ram mode and displays a digit on LED matrix
#
# B. Preparation
#    Connecting ports with LED Matrix should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. LED
#
# D. Note
#    Should use UART1 as a debug log port at stks-a31g314_mmn_touch_a board.
#
# For more information, read a user's manual of the target device carefully.
#
# - LED Matrix (Tested) : stks-a31g314_mmn_touch_a
#
# 1. Channel                    : 0 (LED)
#
# 2. Clock                      : [ 0 m h 24 3 ]
#    Source                     : MCCR (Misc Clock) / HSI (High Speed Internal Clock)
#    Source Divide              : 24
#    LED Pre-Divide             : 3
#
# 3. Config                     : [ 0 i r a 1 255 0 -overlap 8 ]
#    Operation                  : i (Interrupt)
#    Display Mode (RAM / Port)  : r (RAM)
#    Mode                       : a (Alone)
#    Port Share Mode            : 1 (iseq0-iseq1)
#    Pulse Width                : 255 ( Clock / (Width + 1) )
#    Stop Duration              : 0
#    Overlap                    : 8
#
# 4. Icom                       : [ 0 6 7 12 13 15 16 17 18 24 ]
#    Icom Number                : 6, 7, 12, 13, 15, 16, 17, 18, 24
#
# 5. Iseg                       : [ 0 2 4 5 6 7 8 9 ]
#    Iseg Number                : 2, 4, 5, 6, 7, 8, 9
#
# 6. Dim                        : [ 0 7 0 6 0 12 0 13 0 15 0 16 0 17 0 18 0 24 0 ]
#    Icom Number / Dimming      : 7 0, 6 0, 12 0, 13 0, 15 0, 16 0, 17 0, 18 0, 24 0
#
# 7. Data                       : [ 0 7 0x000 6 0x000 12 0x000 13 0x3c4 15 0x044 16 0x3c4 17 0x000 18 0x000 24 0x000 ]
#    Icom Number / ISeg Data    : 7 0x000, ..., 24 0x000
#     - ISeg Data (Bit Order)
#
send ""

send "cm led"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 24 3"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "config 0 i r a 1 255 0 -overlap 8"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "icom 0 6 7 12 13 15 16 17 18 24"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "iseg 0 2 4 5 6 7 8 9"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "dim 0 7 0 6 0 12 0 13 0 15 0 16 0 17 0 18 0 24 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

# num 0
send "data 0 7 0x000 6 0x000 12 0x000 13 0x3c4 15 0x044 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

send "start 0 10000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 1
send "data 0 7 0x000 6 0x000 12 0x000 13 0x000 15 0x3c4 16 0x000 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 2
send "data 0 7 0x000 6 0x000 12 0x000 13 0x1c4 15 0x144 16 0x344 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 3
send "data 0 7 0x000 6 0x000 12 0x000 13 0x144 15 0x144 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 4
send "data 0 7 0x000 6 0x000 12 0x000 13 0x304 15 0x100 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 5
send "data 0 7 0x000 6 0x000 12 0x000 13 0x344 15 0x144 16 0x1c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 6
send "data 0 7 0x000 6 0x000 12 0x000 13 0x3c4 15 0x144 16 0x1c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 7
send "data 0 7 0x000 6 0x000 12 0x000 13 0x004 15 0x004 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 8
send "data 0 7 0x000 6 0x000 12 0x000 13 0x3c4 15 0x144 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 9
send "data 0 7 0x000 6 0x000 12 0x000 13 0x344 15 0x144 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

# num 0
send "data 0 7 0x000 6 0x000 12 0x000 13 0x3c4 15 0x044 16 0x3c4 17 0x000 18 0x000 24 0x000"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 5

send "stop 0"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}

end:
