import logging
import socket
from util import *
import json
from remote_callee import uuid


def main(host='39.96.40.177', port=4222):
    sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)
    sock.settimeout(5)
    ask= 2
    cmd_ask=ask.to_bytes(1, "big")
    id = "wq".encode()
    s = cmd_ask + id
    print (s)
    sock.sendto(s, (host, port))
    data, addr = sock.recvfrom(1024)
    data=data[1:].decode()
    print (data)
    addr=data.split(":")
    peer = (addr[0], int(addr[1]))
    for i in range(10):
        sock.sendto(str(i).encode(), peer)
        data, addr = sock.recvfrom(1024)
        print (data)

if __name__ == '__main__':
    main()
