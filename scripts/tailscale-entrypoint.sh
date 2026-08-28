#!/bin/sh

/usr/local/bin/containerboot &
BOOT_PID=$!

for i in $(seq 1 60); do
    if /usr/local/bin/tailscale status >/dev/null 2>&1; then
        break
    fi
    sleep 2
done

sleep 5

if /usr/local/bin/tailscale funnel status 2>/dev/null | grep -qi 'tailnet only'; then
    echo "enabling tailscale funnel"
    /usr/local/bin/tailscale funnel --bg --https=443 http://app:8080
fi

wait $BOOT_PID
