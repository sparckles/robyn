from watchdog.events import FileSystemEventHandler, FileSystemEvent


class EventHandler(FileSystemEventHandler):
    def __init__(self, file_name: str) -> None:
        pass

    def start_server_first_time(self):  # -> None:
        pass

    def on_any_event(self, event: FileSystemEvent):  # -> None:
        """
        [This function is a callback that will start a new server on every even change]

        :param event [FSEvent]: [a data structure with info about the events]
        """

        pass
