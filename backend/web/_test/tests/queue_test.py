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


def fail(msg):
    """ Used to indicate that a test failed """
    raise AssertionError(msg)


def compare_queues(actual, expected):
    """ Helper function to compare queues """
    if actual["id"] != expected["id"]:
        fail(f"Id differs. Got: {actual['id']}, Expected: {expected['id']}.")
    if actual["workspace_id"] != expected["workspace_id"]:
        fail(f"Workspace_id differs. Got: {actual['workspace_id']}, "
             f"Expected: {expected['workspace_id']}.")
    if actual["info"] != expected["info"]:
        fail(f"Info differs. Got: {actual['info']}, Expected: {expected['info']}.")
    if actual['name'] != expected["name"]:
        fail(f"Name differs. Got: {actual['name']}, Expected: {expected['name']}.")


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
    if response.status_code != 200:
        fail(f"Could not create workspace, got status {response.status_code}")
    return response.json()["id"]


def get_queue_that_does_not_exist():
    """ Trying to get a queue that does not exist should result in a 404 """
    response = requests.get(API_BASE_URL + "queue/12345", headers=admin_headers, verify=False)

    if response.status_code != 404:
        fail(f"Expected status 404 got {response.status_code}")


def create_queue_normal():
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

    if response.status_code != 403:
        fail(f"Expected status 403 got {response.status_code}")


def create_queue_admin():
    """ Creates a workspace and adds a queue to the workspace """
    data = {
        "workspace_id": create_workspace("Workspace Name", "Workspace Info"),
        "name": "Admin Queue",
        "info": "Admins should have permission to create queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=data)

    if response.status_code != 200:
        fail(f"Expected status 200 got {response.status_code}")

    data["id"] = 1
    compare_queues(response.json(), data)


def get_queue():
    """ Creates a workspace with a queue and then get it"""
    data = {
        "workspace_id": create_workspace("Workspace Name", "Workspace Info"),
        "name": "Admin Queue",
        "info": "Admins should have permission to get queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=data)

    if response.status_code != 200:
        fail(f"Could not add queue to workspace, got response {response.status_code}")

    data["id"] = response.json()["id"]

    response = requests.get(f"{API_BASE_URL}/queue/{data['id']}",
                            headers=admin_headers,
                            verify=False)
    if response.status_code != 200:
        fail(f"Expected status 200 got {response.status_code}")

    json = response.json()
    compare_queues(json[0], data)


try:
    get_queue_that_does_not_exist()
    create_queue_normal()
    create_queue_admin()
    get_queue()
    sys.exit(0)

except AssertionError:
    _, exc_value, exc_traceback = sys.exc_info()
    print(traceback.format_tb(exc_traceback, limit=1)[0].splitlines()[-1], end=": ")
    print(exc_value)
    sys.exit(1)
