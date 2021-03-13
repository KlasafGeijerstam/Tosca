#!/usr/bin/python3
import sys
import requests

API_BASE_URL = "https://localhost:25674/api/"
headers = {"Authorization": "Bearer token_admin"}

data = {
    "name": "Eden",
    "info": "Lovely place, nice apples!"
}

try:
    response = requests.post(API_BASE_URL + "workspaces", headers=headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError("Status code:", f"expected 200 got {response.status_code}")

    expected = {
        "name": "Eden",
        "info": "Lovely place, nice apples!",
        "creator": "admin",
    }
    response = requests.get(API_BASE_URL + "workspaces", headers=headers, verify=False)
    if expected["name"] not in [x["name"] for x in response.json()]:
        raise AssertionError(
            "Data:",
            f"expected worspace with name: {expected['name']} to be in response."
            " Got {[x['name'] for x in response.json()]}"
        )

    response = requests.get(API_BASE_URL + "workspaces", headers=headers, verify=False)
    sys.exit(0)

except AssertionError as exception:
    print(exception)
    sys.exit(1)
