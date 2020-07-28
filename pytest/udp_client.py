import logging
import socket
from util import *



def main(host='39.96.40.177', port=4222):
    sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)
    sock.sendto(b'9', (host, port))

    data, addr = sock.recvfrom(1024)
    print('client received: {} {}'.format(addr, data))

    peer = msg_to_addr(data)
    sock.sendto(b'hello from peer', peer)


if __name__ == '__main__':
    main()
