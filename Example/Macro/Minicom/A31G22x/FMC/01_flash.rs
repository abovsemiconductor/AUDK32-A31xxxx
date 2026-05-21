# A. Description
#    A list of commands here configures FMC and erase and write code flash
#
# B. Preparation
#    Setup ARM debugger to connect device without reset to see registers and flash memory
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Flash
#
# For more information, read a user's manual of the target device carefully.
#
# 1. Code Flash Size : 0x20000(MP)/0x40000(MP)
#    0x20000 is taken as flash size to test
# 2. Data Flash Size : 0x8000
# 3. Code Write Protection Size : 0x4000
# 4. Data Write Protection Size : 0x400
# 5. Erase Mode
#    - FMC_CHIP_ERASE_MODE = 0
#    - FMC_PAGE_ERASE_MODE = 1
#    - FMC_1KB_ERASE_MODE  = 2
#    - FMC_2KB_ERASE_MODE  = 4
#    - FMC_4KB_ERASE_MODE  = 8
#    FMC_4KB_ERASE_MODE is taken to test code flash.
#    FMC_1KB_ERASE_MODE is taken to test data flash.

######################  Define Variables ####################### 
# FMC_ID_CFMC = 0
# FMC_ID_DFMC = 1

# FMC_CHIP_ERASE_MODE = 0
# FMC_PAGE_ERASE_MODE = 1
# FMC_1KB_ERASE_MODE  = 2
# FMC_2KB_ERASE_MODE  = 4
# FMC_4KB_ERASE_MODE  = 8

# CODE_FLASH_SIZE  = 131072
# CODE_SECTOR_16KB = 16384
# DATA_FLASH_SIZE  = 32768
# DATA_SECTOR_1KB  = 1024

# CODE_START_TEST_ADR = CODE_FLASH_SIZE - CODE_SECTOR_16KB
# DATA_START_TEST_ADR = DATA_FLASH_SIZE - DATA_SECTOR_1KB
# SIZE_512B  = 512
# SIZE_4KB = 4096

# 0x0E000000 = 234881024
# DATA_START_PHY_ADR = 234881024 + DATA_START_TEST_ADR
# DUMP_ADDR = CODE_START_TEST_ADR
# DUMP_LEN = DATA_SECTOR_1KB
# DUMP_PAUSE = 5
# DISP_MESSAGE = ""

############## Test commands here for Code Flash ##############
timeout 600
send ""

send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}

# Print flash geometric information
print "01.Print flash geometric information"
send ""
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "info"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Unprotect all code/data flash area
print "\n02.Unprotect all code/data flash area"
send ""
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "wprot 0 0xFFFFFFFF 0 0"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "wprot 1 0xFFFFFFFF 0 0"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

# Erase 16KB code flash area at FLASH_SIZE
print "\n03.Erase 16KB code flash area at 0x1C000"
send ""
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "erase 0 0x1C000 8"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "erase 0 0x1D000 8"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "erase 0 0x1E000 8"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 0.1
send "erase 0 0x1F000 8"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n04.Dump memory (0x1C000 ~ 0x1FFFF)"
send "cm scu"
send "rmem 0x1C000 4096" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Erase 1KB data flash area at FLASH_SIZE
print "\n05.Erase 1KB data flash area at 0x7C00"
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "erase 1 0x7C00 2"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n06.Dump memory (0xE007C00 ~ 0xE007FFF)"
send "cm scu"
send "rmem 0xE007C00 256" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at CODE_START_TEST_ADR into code flash write protected area
# See the result of 'info' command
print "\n07.Write 32Byte data (0x00 ~ 0x1F) to code flash write protected area (0x1C000 ~ 0x1FFFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "write 0 0x1C000 0xFFFFFFFF"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n08.Dump memory (0x1C000 ~ 0x1FFFF)"
send "cm scu"
send "rmem 0x1C000 4096" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at DATA_START_TEST_ADR into data flash write protected area
# See the result of 'info' command
print "\n09.Write 32Byte data (0x00 ~ 0x1F) to data flash write protected area (0x7C00 ~ 0x7FFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "write 1 0x7C00 0xFFFFFFFF"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n10.Dump memory (0xE007C00 ~ 0xE007FFF)"
send "cm scu"
send "rmem 0xE007C00 256" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Self Erase (512B) code flash write protect sector at CODE_START_TEST_ADR
print "\n11.Self Erase code flash per 512B (0x1C000 ~ 0x1FFFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "selferase 0 0x1C000"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1C200"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1C400"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1C600"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1C800"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1CA00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1CC00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1CE00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1D000"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1D200"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1D400"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1D600"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1D800"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1DA00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1DC00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1DE00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1E000"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1E200"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1E400"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1E600"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1E800"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1EA00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1EC00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1EE00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1F000"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1F200"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1F400"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1F600"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1F800"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1FA00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 0 0x1FC00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
send "selferase 0 0x1FE00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
sleep 2

# Read 16KB
print "\n12.Dump memory (0x1C000 ~ 0x1FFFFF)"
send ""
expect {
    "<FMC> # "
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

# Self Erase (512B) data flash write protect sector at DATA_START_TEST_ADR
print "\n13.Self Erase data flash per 512B (0x7C00 ~ 0x7FFF)"
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "selferase 1 0x7C00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}

send "selferase 1 0x7E00"
expect {
    "<FMC> # "
    break
    timeout 10 goto end
}
sleep 2

# Read 1KB
print "\n14.Dump memory (0xE007C00 ~ 0xE007FFF)"
send ""
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "cm scu"
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "rmem 0xE007C00 256" 
expect {
    "<SCU> # "
    break
    timeout 120 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at CODE_START_TEST_ADR into code flash write protected area
# See the result of 'info' command
print "\n15.Write 32Byte data (0x00 ~ 0x1F) to code flash write protected area (0x1C000 ~ 0x1FFFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "write 0 0x1C000 0xFFFFFFFF"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n16.Dump memory (0x1C000 ~ 0x1FFFF)"
send "cm scu"
send "rmem 0x1C000 4096" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

# Write predefined 32B data (0x00 ~ 0x1F) at DATA_START_TEST_ADR into data flash write protected area
# See the result of 'info' command
print "\n17.Write 32Byte data (0x00 ~ 0x1F) to data flash write protected area (0x7C00 ~ 0x7FFF)" 
send ""
expect {
    "<SCU> # "
    break
    timeout 5 goto end
}
send "cm fmc"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
send "write 1 0x7C00 0xFFFFFFFF"
expect {
    "<FMC> # "
    break
    timeout 5 goto end
}
sleep 2

print "\n18.Dump memory (0xE007C00 ~ 0xE007FFF)"
send "cm scu"
send "rmem 0xE007C00 256" 
expect {
    "<SCU> #"
    break
    timeout 120 goto end
}
sleep 2

print "\nEnd of Flash Test Sequence"

end:
