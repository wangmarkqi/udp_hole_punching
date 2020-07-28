import logging
import socket
import sys
import json
import threading
import time


uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30"



def main(host='39.96.40.177', port=4222):
    sock = socket.socket(socket.AF_INET, # Internet
                         socket.SOCK_DGRAM) # UDP
    sock.settimeout(5)
    dic = dict(
        cmd="Save",
        callee_uuid=uuid,
    )
    s = json.dumps(dic)

    while True:
        sock.sendto(s.encode(), (host, port))
        try:
            data, addr = sock.recvfrom(1024)
            pac = json.loads(data.decode())
            cmd = pac["cmd"]
            if cmd == "Open":
                print(cmd)
                addr = pac["caller_address"].split(":")
                peer = (addr[0], int(addr[1]))
                s2 = json.dumps(dict(
                    cmd="P2P",
                    msg="from caller",
                ))
                print(peer)
                sock.sendto(s2.encode(), peer)
            elif cmd == "Trans":
                print ("trans====",data)
                addr = pac["callee_address"].split(":")
                peer = (addr[0], int(addr[1]))
                sock.sendto(data, peer)
            else:
                print("cmd not match", cmd)

        except Exception as e:
            print("Timeout!!! Try again...",e)




if __name__ == '__main__':
    main()
