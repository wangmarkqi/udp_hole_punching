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
    save=1
    cmd_save=save.to_bytes(1,"big")
    id= uuid.encode()
    s = cmd_save+id
    sock.sendto(s, (host, port))
    

    while True:
        data, addr = sock.recvfrom(1024*10)
        cmd=data[0]
        res=data[1:].decode()
        print ("&&&&&",cmd,res)
        if cmd == 1:
            pass
        if cmd==3:
            l=res.split(":")
            print ("send hello to peer",l)
            
            sock.sendto(b"hello form peer", (l[0],int(l[1])))
        else:
            print("cmd not match", len(data.decode()))





if __name__ == '__main__':
    main()
