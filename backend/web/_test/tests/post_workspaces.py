#!/usr/bin/python3
import sys
import requests

API_BASE_URL = "https://localhost:25674/api/"
headers = {"Authorization": "Bearer token_admin"}

data = {
    "name": "Eden",
    "info": "Lovely place, nice apples!"
}


def expected_in_response(expected):
    "Check if expected workspace is in response from GET workspaces"
    response = requests.get(API_BASE_URL + "workspaces", headers=headers, verify=False)

    candidates = [x for x in response.json() if x["name"] == expected["name"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name.")

    candidates = [x for x in response.json() if x["creator"] == expected["creator"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name and creator.")

    candidates = [x for x in response.json() if x["info"] == expected["info"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name, creator and info.")


try:
    response = requests.post(API_BASE_URL + "workspaces", headers=headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError("Status code:", f"expected 200 got {response.status_code}")

    expected_in_response({
        "name": "Eden",
        "info": "Lovely place, nice apples!",
        "creator": "admin"
    })

    sys.exit(0)

except AssertionError as exception:
    print(exception)
    sys.exit(1)


