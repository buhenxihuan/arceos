#!/bin/bash

rm ubuntu-20.04-server-cloudimg-amd64.img
cp ubuntu-20.04-server-cloudimg-amd64-origin.img ubuntu-20.04-server-cloudimg-amd64.img
make qemu