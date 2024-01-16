#!/bin/bash

ip link set tun0 up
ip addr add 10.0.0.1/24 dev tun0

ping 10.0.0.2