JH_DIR=~/jailhouse-arceos
JH=$JH_DIR/tools/jailhouse

echo "create axvm zephyr"
sudo $JH axvm create 2 4 ./zephyr-bios.bin ./zephyr.bin