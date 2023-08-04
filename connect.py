import socket
s = socket.socket()
s.connect(("localhost", 8000))

while True:
    try:
        message = input("> ")
        if message == "rickroll":
            with open("./important data.jpg", "rb") as f:
                content = f.read()
                length = str(len(content)).encode("utf8")
                print(length)
                s.send(length)
                s.send(content)
        else:
            content = message
            length = str(len(content)).encode("utf8")
            print(length)
            s.send(length)
            s.send(content.encode("utf8"))
    except KeyboardInterrupt:
        s.close()
        exit()