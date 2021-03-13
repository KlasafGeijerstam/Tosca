#!/usr/bin/python3
import sys
import requests

API_BASE_URL = "https://localhost:25674/api/"
headers = {"Authorization": "Bearer token_normal"}

try:
    response = requests.get(API_BASE_URL + "workspaces", headers=headers, verify=False)

    if response.status_code != 200:
        raise AssertionError("Status code:", f"expected 200 got {response.status_code}")

    if response.json() != []:
        raise AssertionError("Data:", "expected [] got " + f"{response.json()}")

    sys.exit(0)
except AssertionError as exception:
    print(exception)
    sys.exit(1)
