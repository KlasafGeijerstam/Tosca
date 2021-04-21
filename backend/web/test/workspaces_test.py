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
    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)

    candidates = [x for x in response.json() if x["name"] == expected["name"]]
    assert len(candidates) >= 1, "Expected at least one workspace with correct name."

    candidates = [x for x in response.json() if x["creator"] == expected["creator"]]
    assert len(candidates) >= 1, "Expected at least one workspace with correct name and creator."

    candidates = [x for x in response.json() if x["info"] == expected["info"]]
    assert len(candidates) >= 1, "Expected at least one workspace with correct name, creator and info."

    candidates = [x for x in response.json() if x["queues"] == expected["queues"]]
    assert len(candidates) >= 1, "Expected at least one workspace with correct name, creator, info and queues."


def check_response(response, expected):
    """ Helper function to check if response is correct """
    json = response.json()
    assert json["name"] == expected["name"]
    assert json["info"] == expected["info"]
    assert json['creator'] == expected["creator"]


def test_get_empty_workspaces():
    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)

    assert response.status_code == 200
    assert response.json() != []


def test_normal_post():
    data = {
        "name": "Normal workspace",
        "info": "Its just a normal workspace, nothing special."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=normal_headers, verify=False, json=data)
    assert response.status_code == 403


def test_admin_post():
    data = {
        "name": "Eden",
        "info": "Lovely place, nice apples!"
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=data)
    assert response.status_code == 200

    data["creator"] = "admin"
    data["queues"] = []
    check_response(response, data)
    expected_in_response(data)


def test_super_post():
    data = {
        "name": "Krypton",
        "info": "Used to be a quite nice place",
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=super_headers, verify=False, json=data)
    assert response.status_code == 200

    data["creator"] = "super"
    data["queues"] = []
    check_response(response, data)
    expected_in_response(data)


def test_invalid_token_post():
    data = {
        "name": "Black market",
        "info": "Black markets are not allowed on Tosca."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=invalid_headers, verify=False, json=data)
    assert response.status_code == 401


def test_get_workspace_with_queues():
    """
    Create a workspace with two queues, check that we get back both queues when we request the workspace
    """
    data = {
        "name": "get_workspace_with_queues",
        "info": "Description"
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=data)
    assert response.status_code == 200
    workspace_id = response.json()["id"]

    queue1 = {
        "workspace_id": workspace_id,
        "name": "First Queue",
        "info": "First Queue Info"
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=queue1)
    assert response.status_code == 200
    queue1["id"] = response.json()["id"]

    queue2 = {
        "workspace_id": workspace_id,
        "name": "Second Queue",
        "info": "Second Queue Info"
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=queue2)
    assert response.status_code == 200
    queue2["id"] = response.json()["id"]

    response = requests.get(API_BASE_URL + "workspaces", headers=admin_headers, verify=False)
    assert response.status_code == 200
    workspace = [x for x in response.json() if x["id"] == workspace_id][0]

    assert len(workspace["queues"]) == 2
    
    returned_queue1 = [q for q in workspace["queues"] if q["id"] == queue1["id"]][0]
    assert returned_queue1
    assert returned_queue1["name"] == queue1["name"]
    assert returned_queue1["info"] == queue1["info"]

    returned_queue2 = [q for q in workspace["queues"] if q["id"] == queue2["id"]][0]
    assert returned_queue2
    assert returned_queue2["name"] == queue2["name"]
    assert returned_queue2["info"] == queue2["info"]
