# A. Description
#    A list of commands here configures CFMC and erase and write code flash
#
# B. Preparation
#    Setup ARM debugger to connect device without reset to see registers and flash memory
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Flash
#
# For more information, read a user's manual of the target device carefully.

# 1. Code Flash Size : 0x10000(On REQ)/0x20000(MP)/0x40000(MP)
#    0x20000 is taken as flash size to test
# 2. Code Write Protection Size : 0x4000
# 3. Erase Mode
#    - CFMC_CHIP_ERASE_MODE = 0
#    - CFMC_PAGE_ERASE_MODE = 1
#    - CFMC_1KB_ERASE_MODE  = 2
#    - CFMC_2KB_ERASE_MODE  = 4
#    - CFMC_4KB_ERASE_MODE  = 8
#    CFMC_4KB_ERASE_MODE is taken to test code flash.

######################  Define Variables ####################### 
# CFMC_CHIP_ERASE_MODE = 0
# CFMC_PAGE_ERASE_MODE = 1
# CFMC_1KB_ERASE_MODE  = 2
# CFMC_2KB_ERASE_MODE  = 4
# CFMC_4KB_ERASE_MODE  = 8

# FLASH_SIZE  = 131072
# SECTOR_2KB  = 2048
# SECTOR_4KB  = 4096
# SECTOR_16KB = 16384

# START_TEST_ADDR = FLASH_SIZE - SECTOR_16KB
# SIZE_512B  = 512

# DUMP_ADDR = START_TEST_ADDR
# DUMP_LEN = SECTOR_2KB
# DUMP_PAUSE = 5
# DISP_MESSAGE = ""

######################  Test commands here ####################### 
timeout 300
send ""

send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}

# Print flash geometric information
print "01.Print flash geometric information"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "info"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Unprotect all code flash area
print "\n02.Unprotect all code flash area"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "wprot 0xFFFFFFFF 0 0"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Erase 16KB at FLASH_SIZE
print "\n03.Erase 16KB at 0x1C000"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "erase 0x1C000 8"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "erase 0x1D000 8"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "erase 0x1E000 8"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "erase 0x1F000 8"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n04.Dump memory 0x1C000 ~ 0x1FFFF)"
send "cm scu"
send "rmem 0x1C000 4096" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at START_TEST_ADDR into flash write protected area
# See the result of 'info' command
print "\n05.Write 32Byte data (0x00 ~ 0x1F) to flash write protected area 0x1C000 ~ 0x1FFFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "write 0x1C000 0xFFFFFFFF"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Read 2KB
print "\n06.Dump memory (0x1C000 ~ 0x1FFFFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x1C000 4096" 
expect {
    "<SCU> # "
    break
    timeout 120 goto end
}
sleep 2

# Self Erase (512B) write protect sector at START_TEST_ADDR
print "\n07.Self Erase per 512B (0x1C000 ~ 0x1FFFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "selferase 0x1C000"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1C200"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1C400"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1C600"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1C800"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1CA00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1CC00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1CE00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1D000"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1D200"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1D400"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1D600"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1D800"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1DA00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1DC00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1DE00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1E000"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1E200"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1E400"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1E600"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1E800"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1EA00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1EC00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1EE00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1F000"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1F200"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1F400"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1F600"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1F800"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1FA00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}

send "selferase 0x1FC00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
send "selferase 0x1FE00"
expect {
    "<CFMC> # "
    break
    timeout 10 goto end
}
sleep 2

# Read 16KB
print "\n08.Dump memory (0x1C000 ~ 0x1FFFFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x1C000 4096" 
expect {
    "<SCU> # "
    break
    timeout 120 goto end
}
sleep 2

# Self-Write predefined 32B data (0x00 ~ 0x1F) at START_TEST_ADDR into flash write protected area
print "\n09.Self Write 32Byte data (0x00 ~ 0x1F) to flash write protected area (0x1C000 ~ 0x1FFFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm cfmc"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "selfwrite 0x1C000 0xFFFFFFFF"
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Read 16KB
print "\n10.Dump memory (0x1C000 ~ 0x1FFFFF)"
send ""
expect {
    "<CFMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0x1C000 4096" 
expect {
    "<SCU> # "
    break
    timeout 120 goto end
}

print "\nEnd of Flash Test Sequence"

end:

