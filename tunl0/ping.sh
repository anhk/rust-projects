#!/bin/bash

if [ "$(uname)" == "Darwin" ]; then
    ip link set utun6 up 
    ip route add 10.10.10.0/24 dev utun6
    ping 10.10.10.22
else
    ip link set tun0 up
    ip addr add 10.0.0.1/24 dev tun0
    ping 10.0.0.2
fi

