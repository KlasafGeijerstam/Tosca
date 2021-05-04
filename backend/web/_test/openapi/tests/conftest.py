from openapi_core.validation.request.validators import RequestValidator
from openapi_core.validation.response.validators import ResponseValidator
from openapi_core.contrib.requests import RequestsOpenAPIResponse, RequestsOpenAPIRequest
from openapi_core import create_spec
from openapi_spec_validator import validate_spec
from openapi_spec_validator.readers import read_from_filename
import openapi_core
import pytest
from requests import Request, Session
from requests import PreparedRequest
from pathlib import Path
from os import environ
import yaml

OPENAPI_SPEC = environ['OPENAPI_SPEC']
spec_dict, spec_url = read_from_filename(OPENAPI_SPEC)
validate_spec(spec_dict)

api_spec = create_spec(spec_dict)

BASE = 'https://localhost:25674/api'
AUTH = {"Authorization": "Bearer token_admin"}

session = Session()

def request(method: str, url: str, body: dict = None):

    request = Request(method, BASE + url, headers=AUTH)
    if body is not None:
        request.json = body

    openapi_request = RequestsOpenAPIRequest(request)

    req = request.prepare()

    resp = session.send(req, verify=False)

    openapi_response = RequestsOpenAPIResponse(resp)
    validator = ResponseValidator(api_spec)
    result = validator.validate(openapi_request, openapi_response)

    if result.errors != []:
        print(result.errors)

    assert result.errors == []

