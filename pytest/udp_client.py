import logging
import numpy as np
import socket
from util import *

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
host='39.96.40.177'
port=4222

def sess_cmd_id(cmd):
    cmd=cmd.to_bytes(1,"big")
    id=b"asdfasdf"
    return cmd+id

def test_cmd(cmd):
    data=sess_cmd_id(cmd)
    sock.sendto(data, (host, port))
    res, addr = sock.recvfrom(1024)
    cmd=res[0]
    print (cmd,str(res[1:]))
test_cmd(1)
test_cmd(2)

