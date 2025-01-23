import customtkinter as ctk
from osas.osas import Player
from enum import Enum
from mutagen.mp3 import MP3

import const


class State(Enum):
    Play = 1
    Stop = 2
    Pause = 3


class Control(ctk.CTkFrame):
    state = State.Stop
    __current_song: str | None = None

    def __init__(self, player: Player, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.player = player

        self.scale = ctk.CTkSlider(self, command=self.seek)
        self.next_button = ctk.CTkButton(self, text="next", width=10)
        self.play_button = ctk.CTkButton(
            self, text="play", command=self.__play, width=10
        )
        self.prev_button = ctk.CTkButton(self, text="prev", width=10)

        self.scale.pack(side="top", fill="x", pady="10")
        self.next_button.pack(side="left")
        self.play_button.pack(side="left")
        self.prev_button.pack(side="left")

    def __play(self):
        if self.__current_song and self.state is State.Stop:
            self.player.play(self.__current_song)
            self.state = State.Play
            self.scale._to = MP3(self.__current_song).info.length
            self.scale.set(0)
        elif self.__current_song and (
            self.state is State.Play or self.state is State.Pause
        ):
            self.player.pause()
            self.state = State.Pause if self.player.is_paused() else State.Play

    def seek(self, pos: float):
        self.player.seek(int(pos * const.SECOND_MS))

    def set_current_song(self, path: str):
        self.__current_song = path
