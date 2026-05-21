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
#    1. I2C0    : PD0 (Serial Clock)
#               : PD1 (Serial Data)
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
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 0 a 1 -pupd p ] [ 3 1 a 1 -pupd p ]
#    Pin Number                : 0 / 1 
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

send "port 3 0 a 1 -pupd p"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 1 a 1 -pupd p"
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
