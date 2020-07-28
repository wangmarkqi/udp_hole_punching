import logging
import socket
from util import *
import json
from remote_callee import uuid


def main(host='39.96.40.177', port=4222):
    sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)
    sock.settimeout(5)
    dic=dict(
        cmd="Open",
        callee_uuid=uuid,
    )
    s=json.dumps(dic)

    sock.sendto(s.encode(), (host, port))

    data, addr = sock.recvfrom(1024)
    pac=json.loads(data)
    addr=pac["callee_address"].split(":")
    peer = (addr[0], int(addr[1]))
    print (peer)
    tran = dict(
        cmd="P2P",
        msg="from caller"
    )
    sock.sendto(json.dumps(tran).encode(), peer)
    for i in ["a","b","c"]:
        tran['msg']=tran['msg']+str(i)
        sock.sendto(json.dumps(tran).encode(), peer)
        try:
            data, addr = sock.recvfrom(1024)
            print(11111, data, addr)
        except:
            print ("time out")


if __name__ == '__main__':
    main()
