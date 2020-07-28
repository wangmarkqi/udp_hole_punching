import logging
import socket
import sys
from util import *

logger = logging.getLogger()


def main(host='39.96.40.177', port=4222):
    sock = socket.socket(socket.AF_INET, # Internet
                         socket.SOCK_DGRAM) # UDP
    sock.sendto(b'0', (host, port))

    while True:
        data, addr = sock.recvfrom(1024)
        print('client received first : {} {}'.format(addr, data))
        addr = msg_to_addr(data)
        print ("addr===",addr)
        sock.sendto(b'1', addr)
        data, addr = sock.recvfrom(1024)
        print('client received second: {} {}'.format(addr, data))


if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(message)s')
    main()
