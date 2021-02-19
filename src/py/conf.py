import os
import sys
import json


class Conf:
    """Define configuration to be used across modules."""

    BASE = {
      "host": "192.168.1.x",
      "web_port": 8080,
      "led_port": 21324,
      "devices": [
        {
          "name": "my_light_strip",
          "host": "192.168.1.x",
          "port": 21324,
          "recv_from": ["192.168.1.x", "192.168.1.x"]
        },
        {
          "name": "other_light_strip",
          "host": "192.168.1.x",
          "port": 21324,
          "recv_from": ["192.168.1.x", "192.168.1.x"]
         }
      ]
    }

    def __init__(self):
        try:
            with open(os.path.join("routes.json")) as f:
                config = json.load(f)
        except FileNotFoundError:
            with open("routes.json", "w") as conf:
                json.dump(Conf.BASE, conf, indent=4)
            config = Conf.BASE

        config['wwwDirectory'] = os.path.join(os.getcwd(), "www/")
        config['templateDirectory'] = os.path.join(config['wwwDirectory'], "templates/")
        config['staticDirectory'] = os.path.join(config['wwwDirectory'], "static/")

        for key, value in config.items():
            setattr(self, key, value)


conf = Conf()
