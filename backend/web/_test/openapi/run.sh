#!/bin/sh
export OPENAPI_SPEC=../../swagger/api.yaml 
export PYTHONWARNINGS="ignore:Unverified HTTPS request"
pytest .
