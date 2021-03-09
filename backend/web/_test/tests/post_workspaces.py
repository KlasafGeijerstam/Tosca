#!/usr/bin/python3
import requests
import json

api_base_url = f"https://localhost:25674/api/"
headers = { "Authorization": "Bearer token_admin" }

data = {
    "name": "Eden",
    "info": "Lovely place, nice apples!"
}

try:
    response = requests.post(api_base_url + "workspaces", headers=headers, verify=False, json=data)
    if response.status_code != 200:
        raise RuntimeError("Status code:", f"expected 200 got {response.status_code}") 
    
    exit(0)

except RuntimeError as e:
    print("\t" + f"{e}")
    exit(1)
