#!/usr/bin/env python3
from socket import socket, AF_UNIX

with socket(AF_UNIX) as sock:
    #TODO use XDG_RUNTIME_DIR instead of hardcoding the path and the user id
    sock.connect("/var/run/user/1000/feola/socket")
    sock.send("{'message_type':'query', 'data':'hello'}".encode())

