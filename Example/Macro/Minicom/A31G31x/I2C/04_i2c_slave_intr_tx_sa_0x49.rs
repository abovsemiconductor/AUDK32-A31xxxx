# A. Description
#    A list of commands here configures I2C module as a slave and transmits data via I2C.
#
# B. Preparation
#    Connecting ports with an external I2C device should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. I2C
#
# D. Default Port
#    1. I2C0    : PD0 (Serial Clock)
#               : PD1 (Serial Data)
#
# For more information, read a user's manual of the target device carefully.
#
# I2C0
# 1. Channel                   : 0 (I2C0)
#
# 2. Config                    : [ 0 i s 400000 -saddr 0x49]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Master)
#    Baudrate                  : 400000 (400KHz)
#    Slave Address             : 0x49 (Hexa Format)
#
# 3. Data                      : [ 0 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0 ]
#    Data Length               : 8
#    Data                      : 0xa5 ... (Hexa and space (delimitor))
#
# 4. Tx                        : [ 0 8 ]
#    Receive Data Length       : 8
#
# I2C0
send ""

send "cm i2c"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "config 0 i s 400000 -saddr 0x49"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "data 0 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "tx 0 8"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

end:
