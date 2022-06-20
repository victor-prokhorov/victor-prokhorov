#!/usr/bin/env bash

if [[ $# -ge 3 ]]; then
    export CDK_DEPLOY_ACCOUNT=$1
    export CDK_DEPLOY_REGION=$2
    export CONNECTION_ARN=$3
    shift; shift; shift
    npx cdk deploy "$@" 
    exit $?
else
    exit 1
fi