import osas
import time
import threading


def main():
    player = osas.osas.Player()
    player.play("/home/joe/Music/Tselatra/Kamboty.mp3")
    time.sleep(5)
    print("try seek the song")
    player.seek(10000)
    print("seeked")

    th = threading.Thread(target=lambda: player.sleep_until_end())
    th.start()


if __name__ == "__main__":
    main()
