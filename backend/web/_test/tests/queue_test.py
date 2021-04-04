#!/usr/bin/python3
import sys
import traceback
import requests

API_BASE_URL = "https://localhost:25674/api/"
admin_headers = {"Authorization": "Bearer token_admin"}
normal_headers = {"Authorization": "Bearer token_normal"}
super_headers = {"Authorization": "Bearer token_super"}
invalid_headers = {"Authorization": "Bearer an_odd_token"}


def check_response(response, expected):
    """ Helper function to check if response is correct """
    json = response.json()
    if json["id"] != expected["id"]:
        raise AssertionError(f"Returned id differs. Got: {json['id']}, Expected: {expected['id']}.")
    if json["workspace_id"] != expected["workspace_id"]:
        raise AssertionError(f"Returned workspace_id differs. Got: {json['workspace_id']}, Expected: {expected['workspace_id']}.")
    if json["info"] != expected["info"]:
        raise AssertionError(f"Returned info differs. Got: {json['info']}, Expected: {expected['info']}.")
    if json['name'] != expected["name"]:
        raise AssertionError(f"Returned name differs. Got: {json['name']}, Expected: {expected['name']}.")


def get_queue_that_does_not_exist():
    response = requests.get(API_BASE_URL + "queue/12345", headers=admin_headers, verify=False)

    if response.status_code != 404:
        raise AssertionError(f"Expected status 404 got {response.status_code}")


def put_queue_normal():
    data = {
        "workspace_id": 1234,  # Irrelevant
        "name": "Normals Queue",
        "info": "Normals should not have permission to create queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=normal_headers, verify=False, json=data)
    
    if response.status_code != 403:
        raise AssertionError(f"Expected status 403 got {response.status_code}")


def put_queue_admin():
    ws_data = {
        "name": "Workspace1",
        "info": "Workspace 1 info."
    }
    response = requests.post(API_BASE_URL + "workspaces", headers=admin_headers, verify=False, json=ws_data)
    if response.status_code != 200:
        raise AssertionError(f"Could not create workspace, got status {response.status_code}")

    data = {
        "workspace_id": response.json()["id"],
        "name": "Admin Queue",
        "info": "Admins should have permission to create queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=admin_headers, verify=False, json=data)
    
    if response.status_code != 200:
        raise AssertionError(f"Expected status 200 got {response.status_code}")
    
    data["id"] = 1
    check_response(response, data)
    

try:
    get_queue_that_does_not_exist()
    put_queue_normal()
    put_queue_admin()
    sys.exit(0)

except AssertionError:
    _, exc_value, exc_traceback = sys.exc_info()
    print(traceback.format_tb(exc_traceback, limit=1)[0].splitlines()[-1], end=": ")
    print(exc_value)
    sys.exit(1)
