import customtkinter as ctk
import tkinter as tk
import const
import widgets
import threading
import time

from widgets import State
from osas import osas


class Application(ctk.CTk):
    run = True
    player = osas.Player()

    def __init__(self):
        super().__init__()
        self.geometry(f"{const.W}x{const.H}")

        self.container = ctk.CTkFrame(self)

        self.left = ctk.CTkFrame(self.container, width=const.W / 2)
        self.listbox = tk.Listbox(
            self.left,
            bg="white",
            fg="gray",
            height=80,
            width=60,
            selectbackground="black",
            selectforeground="white",
        )
        self.listbox.pack(fill="x")
        self.left.pack(side="left", fill="y")

        self.right = ctk.CTkFrame(self.container, width=const.W / 2)
        self.control = widgets.Control(self.player, self.right)
        self.control.pack(pady="10", fill="x")
        self.right.pack(side="left", fill="y")

        self.container.pack(fill="y", anchor="center")

        self.th_update = threading.Thread(target=self.update)
        self.th_update.start()

    def update(self) -> None:
        while self.run:
            if self.control.state == State.Play:
                self.control.scale.set(self.player.get_pos() / const.SECOND_MS)
            time.sleep(1)


if __name__ == "__main__":
    application = Application()
    application.control.set_current_song(
        "/home/joe/Music/Tselatra/Kamboty.mp3",
    )
    application.mainloop()
    application.run = False
