# A. Description
#    A list of commands here reads/writes data from/to an external SRAM via EBI module.
#	 And it also contains commands to setup EBI.
#
# B. Preparation
#    Connecting ports with SRAM should be correct.
#
# For more information, read a user's manual of the target device carefully.
#
# - SRAM Kit (Tested) : IS62WV51216BLL SRAM Board (ISSI)
#
# 1. Config                     : [ 0 m 8 e 1 31 ]
#    Bus Mode                   : m (Muxed)
#    Bus Width                  : 8 (8bit)
#    Byte Lane                  : e (Enable)
#    Keep Address Clock N-Cycle : 1 (1-Cycle)
#    Read/Write N-Wait          : 31 (0 Wait)
#
# 2. Write                      : [ 0 0x60000000 0x5A b ]
#    Address                    : 0x60000000
#    Data                       : 0x5A
#    Data Width                 : b (Byte)
#
# 3. Read                       : [ 0 0x60000000 b ]
#    Address                    : 0x60000000
#    Data Width                 : b (Byte)
#
send ""

send "cm ebi"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
send "uninit 0"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
send "init 0"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
send "config 0 m 8 e 1 31"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
send "write 0 0x60000000 0x5A b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0xFF b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0xA5 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0x5A5A h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0xdeaf h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0xA5A5 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "write 0 0x60000000 0xdeaf5AA5 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 b"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 h"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}
sleep 1
send "read 0 0x60000000 w"
expect {
    "<EBI> # "
    break
    timeout 5 goto end
}

end:
