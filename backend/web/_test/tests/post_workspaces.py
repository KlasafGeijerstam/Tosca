#!/usr/bin/python3
import sys
import requests

API_BASE_URL = "https://localhost:25674/api/"
admin_headers = {"Authorization": "Bearer token_admin"}
normal_headers = {"Authorization": "Bearer token_normal"}
super_headers = {"Authorization": "Bearer token_super"}
invalid_headers = {"Authorization": "Bearer an_odd_token"}


def expected_in_response(expected):
    """ Helper function to check if a workspace is a part of the response """
    "Check if expected workspace is in response from GET workspaces"
    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)

    candidates = [x for x in response.json() if x["name"] == expected["name"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name.")

    candidates = [x for x in response.json() if x["creator"] == expected["creator"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name and creator.")

    candidates = [x for x in response.json() if x["info"] == expected["info"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name, creator and info.")


def normal_post():
    data = {
        "name": "Normal workspace",
        "info": "Its just a normal workspace, nothing special."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=normal_headers, verify=False, json=data)
    if response.status_code != 401:
        raise AssertionError(f"Posting as normal user, expected status 401 got {response.status_code}")


def admin_post():
    data = {
        "name": "Eden",
        "info": "Lovely place, nice apples!"
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError(f"Posting as admin user, expected status 200 got {response.status_code}")

    data["creator"] = "admin"
    expected_in_response(data)


def super_post():
    data = {
        "name": "Krypton",
        "info": "Used to be a quite nice place",
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=super_headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError(f"Posting as super user, expected status 200 got {response.status_code}")

    data["creator"] = "super"
    expected_in_response(data)


def invalid_token_post():
    data = {
        "name": "Black market",
        "info": "Black markets are not allowed on Tosca."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=normal_headers, verify=False, json=data)
    if response.status_code != 401:
        raise AssertionError(f"Posting with invalid token, expected status 401 got {response.status_code}")


try:
    admin_post()
    normal_post()
    super_post()
    invalid_token_post()
    sys.exit(0)

except AssertionError as exception:
    print(exception)
    sys.exit(1)


