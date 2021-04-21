#!/usr/bin/python3
import requests
import json
from time import sleep

base_url = "https://localhost:25674/ping"

backend_up = False
while not backend_up:
    try:
        print(f"Trying to connect to backend at {base_url}")
        response = requests.get(base_url, verify=False)
        backend_up = response.status_code == 200
        sleep(1)
    except requests.exceptions.RequestException as e:
        print(str(e))
        backend_up = False
        sleep(1)
    except Exception as e:
        print("Some other Exception" + str(e))
        exit(1)

