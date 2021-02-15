#!/usr/bin/python3
import requests
import json

api_base_url = f"https://localhost:25674/api/"

try:
    response = requests.get(api_base_url + "workspaces", verify=False)
    if response.status_code != 200:
        raise RuntimeError("Status code:", f"expected 200 got {response.status_code}") 
    
    if response.json() != []:
        raise RuntimeError("Data:", "expected [] got " + f"{response.json()}") 

    exit(0)
except RuntimeError as e:
    print("\t" + f"{e}")
    exit(1)
