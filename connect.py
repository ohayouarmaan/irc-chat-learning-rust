import socket
s = socket.socket()
s.connect(("localhost", 8000))

s.send('wow'.encode("utf8"))
while True:
    try:
        message = input("> ")
        s.send(message.encode("utf8"))
    except KeyboardInterrupt:
        s.close()
        exit()