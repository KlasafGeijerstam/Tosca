from conftest import request

def test_get_workspaces():
    request('GET', '/workspaces')
