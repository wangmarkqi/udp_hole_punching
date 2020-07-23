import logging
import socket
from util import *



def main(host='127.0.0.1', port=9999):
    sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)
    sock.sendto(b'0', (host, port))

    while True:
        data, addr = sock.recvfrom(1024)
        print('client received: {} {}'.format(addr, data))


if __name__ == '__main__':
    main()
