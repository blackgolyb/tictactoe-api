#!/bin/bash

ls -la /app
chown -R user:user /app/store

echo "$HOST_PROD, $HOST, $PORT"
gosu user autossh -M 0 -R $HOST_PROD:80:$HOST:$PORT serveo.net &
exec gosu user /app/tic_tac_toe_api
