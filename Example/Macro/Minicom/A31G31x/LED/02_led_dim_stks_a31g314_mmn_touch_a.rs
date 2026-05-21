# A. Description
#    A list of commands here configures LED module as ram mode and controls LED dimming on LED matrix.
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
# 7. Data                       : [ 0 7 0x3F4 6 0x3F4 12 0x3F4 13 0x3F4 15 0x3F4 16 0x3F4 17 0x3F4 18 0x3F4 24 0x3F4 ]
#    Icom Number / ISeg Data    : 7 0x3F4, ..., 24 0x3F4
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

send "data 0 7 0x3F4 6 0x3F4 12 0x3F4 13 0x3F4 15 0x3F4 16 0x3F4 17 0x3F4 18 0x3F4 24 0x3F4"
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

send "dim 0 7 1 6 1 12 1 13 1 15 1 16 1 17 1 18 1 24 1"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 50 6 50 12 50 13 50 15 50 16 50 17 50 18 50 24 50"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 100 6 100 12 100 13 100 15 100 16 100 17 100 18 100 24 100"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 150 6 150 12 150 13 150 15 150 16 150 17 150 18 150 24 150"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 200 6 200 12 200 13 200 15 200 16 200 17 200 18 200 24 200"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 255 6 255 12 255 13 255 15 255 16 255 17 255 18 255 24 255"
expect {
    "<LED> # "
    break
    timeout 5 goto end
}
sleep 1

send "dim 0 7 0 6 0 12 0 13 0 15 0 16 0 17 0 18 0 24 0"
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
