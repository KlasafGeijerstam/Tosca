#!/usr/bin/python3
import sys
import traceback 
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

    candidates = [x for x in response.json() if x["queues"] == expected["queues"]]
    if len(candidates) < 1:
        raise AssertionError("Expected at least one workspace with correct name, creator, info and queues.")


def check_response(response, expected):
    """ Helper function to check if response is correct """
    json = response.json()
    if json["name"] != expected["name"]:
        raise AssertionError(f"Returned name differs. Got: {json['name']}, Expected: {expected['name']}.")
    if json["info"] != expected["info"]:
        raise AssertionError(f"Returned info differs. Got: {json['info']}, Expected: {expected['info']}.")
    if json['creator'] != expected["creator"]:
        raise AssertionError(f"Returned creator differs. Got: {json['creator']}, Expected: {expected['creator']}.")

def get_empty_workspaces():
    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)

    if response.status_code != 200:
        raise AssertionError(f"Expected status 200 got {response.status_code}")

    if response.json() != []:
        raise AssertionError(f"Expected [] got {response.json()}")


def normal_post():
    data = {
        "name": "Normal workspace",
        "info": "Its just a normal workspace, nothing special."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=normal_headers, verify=False, json=data)
    if response.status_code != 403:
        raise AssertionError(f"Posting as normal user, expected status 403 got {response.status_code}")


def admin_post():
    data = {
        "name": "Eden",
        "info": "Lovely place, nice apples!"
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError(f"Posting as admin user, expected status 200 got {response.status_code}")

    data["creator"] = "admin"
    data["queues"] = []
    check_response(response, data)
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
    data["queues"] = []
    check_response(response, data)
    expected_in_response(data)


def invalid_token_post():
    data = {
        "name": "Black market",
        "info": "Black markets are not allowed on Tosca."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=invalid_headers, verify=False, json=data)
    if response.status_code != 401:
        raise AssertionError(f"Posting with invalid token, expected status 401 got {response.status_code}")


def get_workspace_with_queues():
    """
    Create a workspace with two queues, check that we get back both queues when we request the workspace
    """
    data = {
        "name": "get_workspace_with_queues",
        "info": "Description"
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=data)
    if response.status_code != 200:
        raise AssertionError(f"Could not create workspace, got status {response.status_code}")
    workspace_id = response.json()["id"]

    queue1 = {
        "workspace_id": workspace_id,
        "name": "First Queue",
        "info": "First Queue Info"
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=queue1)
    if response.status_code != 200:
        raise AssertionError(f"Could not create queue, got status {response.status_code}")
    queue1["id"] = response.json()["id"]

    queue2 = {
        "workspace_id": workspace_id,
        "name": "Second Queue",
        "info": "Second Queue Info"
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=queue2)
    if response.status_code != 200:
        raise AssertionError(f"Could not create queue, got status {response.status_code}")
    queue2["id"] = response.json()["id"]

    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)
    if response.status_code != 200:
        raise AssertionError(f"Could not get workspaces, got status {response.status_code}")
    workspace = [x for x in response.json() if x["id"] == workspace_id][0]

    if len(workspace["queues"]) != 2:
        raise AssertionError(f"Incorrect number of queues, expected 2, got {len(workspace['queues'])}")
    
    returned_queue1 = [q for q in workspace["queues"] if q["id"] == queue1["id"]][0]
    if not returned_queue1:
        raise AssertionError(f"Could not find queue1 in queues, got {workspace['queues']}")

    if returned_queue1["name"] != queue1["name"]:
        raise AssertionError( f"Invalid name for queue 1, got: \"{returned_queue1['name']}\", expected: \"{queue1['name']}\"")

    if returned_queue1["info"] != queue1["info"]:
        raise AssertionError( f"Invalid info for queue 1, got: \"{returned_queue1['info']}\", expected: \"{queue1['info']}\"")

    returned_queue2 = [q for q in workspace["queues"] if q["id"] == queue2["id"]][0]
    if not returned_queue2:
        raise AssertionError(f"Could not find queue2 in queues, got {workspace['queues']}")

    if returned_queue2["name"] != queue2["name"]:
        raise AssertionError( f"Invalid name for queue 2, got: \"{returned_queue2['name']}\", expected: \"{queue2['name']}\"")

    if returned_queue2["info"] != queue2["info"]:
        raise AssertionError( f"Invalid info for queue 2, got: \"{returned_queue2['info']}\", expected: \"{queue2['info']}\"")


try:
    get_empty_workspaces()
    admin_post()
    normal_post()
    super_post()
    invalid_token_post()
    get_workspace_with_queues()
    sys.exit(0)

except AssertionError:
    _, exc_value, exc_traceback = sys.exc_info()
    print(traceback.format_tb(exc_traceback, limit=1)[0].splitlines()[-1], end=": ")
    print(exc_value)
    sys.exit(1)
