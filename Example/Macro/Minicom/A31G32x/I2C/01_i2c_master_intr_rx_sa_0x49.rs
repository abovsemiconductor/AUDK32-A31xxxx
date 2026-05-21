# A. Description
#    A list of commands here configures I2C module as a master and receives data via I2C.
#
# B. Preparation
#    Connecting ports with an external I2C device should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. I2C
#    2. PCU/GPIO
#
# D. Default Port
#    1. I2C0    : PB8 (Serial Clock)
#               : PB9 (Serial Data)
#
# For more information, read a user's manual of the target device carefully.
#
# I2C0
# 1. Channel                   : 0 (I2C0)
#
# 2. Config                    : [ 0 i m 400000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Master)
#    Baudrate                  : 400000 (400KHz)
#
# 3. Rx                        : [ 0 8 -saddr 0x49 ]
#    Receive Data Length       : 8
#    Slave Address             : 0x49 (Hexa Format)
#
# PCU (I2C0)
# 1. Port Group                : 1 (PCU Port B)
#
# 2. Port                      : [ 1 8 a 1 -pupd p ] [ 1 9 a 1 -pupd p ]
#    Pin Number                : 8 / 9 
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (SCL/SDA)
#    Pull-up/down              : p (Pull-up)
#
# PCU
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 8 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 9 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# I2C0
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

send "config 0 i m 400000"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

send "rx 0 8 -saddr 0x49"
expect {
    "<I2C> # "
    break
    timeout 5 goto end
}

end:
