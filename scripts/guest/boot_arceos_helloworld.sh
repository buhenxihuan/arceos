JH_DIR=~/jailhouse-arceos
JH=$JH_DIR/tools/jailhouse

echo "create axvm arceos"
sudo $JH axvm create 2 3 ./arceos-bios.bin ./helloworld_x86_64-qemu-q35.bin