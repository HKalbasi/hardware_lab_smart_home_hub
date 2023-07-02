from time import sleep
import threading
import json
import requests

lock = threading.Lock()
devices = []

def send(val):
    print(json.dumps(val), flush=True)

class HeartBeatThread(threading.Thread):
    def run(self):
        while True:
            send({ "type": "heart_beat" })
            sleep(0.5)

class DeviceDetectorThread(threading.Thread):
    def run(self):
        r = requests.get(url="http://192.168.43.1:8080/conf")
        r = json.loads(r.text.replace('=', ':'))
        for d in r["sensors"]:
            d["location"] = r["room"]
            d["kind"] = "sensor"
            d["ip"] = "192.168.43.1"
            d["value"] = ""
            with lock:
                devices.append(d)
            send({ "type": "new_device", "value": d })
        for d in r["actuators"]:
            d["location"] = r["room"]
            d["kind"] = "actuator"
            d["ip"] = "192.168.43.1"
            d["value"] = ""
            with lock:
                devices.append(d)
            send({ "type": "new_device", "value": d })

class DevicePollerThread(threading.Thread):
    def run(self):
        while True:
            sleep(0.7)
            with lock:
                for d in devices:
                    r = requests.get(url=f"http://{d['ip']}:8080{d['uri']}")
                    if d["value"] != r.text:
                        d["value"] = r.text
                        send({ "type": "update_device", "value": d })

for t in [HeartBeatThread(), DeviceDetectorThread(), DevicePollerThread()]:
    t.start()
