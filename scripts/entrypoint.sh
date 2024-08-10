#!/bin/bash

ls -la /app

echo "$HOST_PROD, $HOST, $PORT"
autossh -M 0 -R $HOST_PROD:80:$HOST:$PORT serveo.net &
/app/tic_tac_toe_api