#!/usr/bin/python3
import sys
import traceback
import requests

API_BASE_URL = "https://localhost:25674/api/"
admin_headers = {"Authorization": "Bearer token_admin"}
normal_headers = {"Authorization": "Bearer token_normal"}
super_headers = {"Authorization": "Bearer token_super"}
invalid_headers = {"Authorization": "Bearer an_odd_token"}


def get_queue_that_does_not_exist():
    response = requests.get(API_BASE_URL + "queue/12345", headers=admin_headers, verify=False)

    if response.status_code != 404:
        raise AssertionError(f"Expected status 404 got {response.status_code}")


def put_queue_normal():
    data = {
        "workspace_id": 1234,  # Irrelevant
        "name": "Normals Queue",
        "info": "Normals should not have permissions to create queues."
    }
    response = requests.post(API_BASE_URL + "queue", headers=normal_headers, verify=False, json=data)
    
    if response.status_code != 403:
        raise AssertionError(f"Expected status 403 got {response.status_code}")


try:
    get_queue_that_does_not_exist()
    put_queue_normal()
    sys.exit(0)

except AssertionError:
    _, exc_value, exc_traceback = sys.exc_info()
    print(traceback.format_tb(exc_traceback, limit=1)[0].splitlines()[-1], end=": ")
    print(exc_value)
    sys.exit(1)
