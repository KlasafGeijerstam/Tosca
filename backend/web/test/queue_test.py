#!/usr/bin/python3
""" Tests for the api/queue endpoint. Uses api/workspaces to create new workspaces """
import sys
import traceback
import requests

API_BASE_URL = "https://localhost:25674/api/"
admin_headers = {"Authorization": "Bearer token_admin"}
normal_headers = {"Authorization": "Bearer token_normal"}
super_headers = {"Authorization": "Bearer token_super"}
invalid_headers = {"Authorization": "Bearer an_odd_token"}


def compare_queues(actual, expected):
    """ Helper function to compare queues """
    assert actual["id"] == expected["id"]
    assert actual["workspace_id"] == expected["workspace_id"]
    assert actual["info"] == expected["info"]
    assert actual['name'] == expected["name"]


def create_workspace(name, info):
    """ Helper function, creates a workspace and returns the workspace_id """
    ws_data = {
        "name": name,
        "info": info,
    }
    response = requests.post(
        API_BASE_URL + "workspaces",
        headers=admin_headers,
        verify=False,
        json=ws_data
    )
    assert response.status_code == 200
    return response.json()["id"]


def test_get_queue_that_does_not_exist():
    """ Trying to get a queue that does not exist should result in a 404 """
    response = requests.get(API_BASE_URL + "queue/12345", headers=admin_headers, verify=False)

    assert response.status_code == 404


def test_create_queue_normal():
    """ A normal user should not be allowed to create a queue, should result in 403 """
    data = {
        "workspace_id": 1234,  # Irrelevant
        "name": "Normals Queue",
        "info": "Normals should not have permission to create queues."
    }
    response = requests.post(
        API_BASE_URL + "queue",
        headers=normal_headers,
        verify=False,
        json=data
    )

    assert response.status_code == 403


def test_create_queue_admin():
    """ Creates a workspace and adds a queue to the workspace """
    data = {
        "workspace_id": create_workspace("Workspace Name", "Workspace Info"),
        "name": "Admin Queue",
        "info": "Admins should have permission to create queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=data)

    assert response.status_code == 200

    data["id"] = response.json()["id"]
    compare_queues(response.json(), data)


def test_get_queue():
    """ Creates a workspace with a queue and then get it"""
    data = {
        "workspace_id": create_workspace("Workspace Name", "Workspace Info"),
        "name": "Admin Queue",
        "info": "Admins should have permission to get queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=data)

    assert response.status_code == 200

    data["id"] = response.json()["id"]

    response = requests.get(f"{API_BASE_URL}/queue/{data['id']}",
                            headers=admin_headers,
                            verify=False)
    assert response.status_code == 200

    json = response.json()
    compare_queues(json[0], data)
